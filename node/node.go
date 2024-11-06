package node

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os/exec"

	"github.com/CelestialCrafter/fenlu/common"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
	"github.com/puzpuzpuz/xsync/v3"
)

type Node struct {
	writer io.Writer
	reader io.Reader
	name string
	pendingRequests *xsync.MapOf[int, chan *protocol.Response]
	Capabilities map[string]struct{}
}

func (n *Node) responseReader() {
	decoder := json.NewDecoder(n.reader)
	for {
		response := new(protocol.Response)
		err := decoder.Decode(response)
		if err != nil {
			if errors.Is(err, io.EOF) {
				break
			}

			log.Warn("could not decode response", "error", err)
			continue
		}

		if response.Result == nil && response.Error == "" {
			log.Warn("request did not have response or error: ", "response", response)
			continue
		}


		channel, ok := n.pendingRequests.Load(response.ID)
		if !ok {
			log.Warn("received response with no request pending", "response", response)
			continue
		}

		channel <- response
	}
}

func InitializeNode(name string, cmds []*exec.Cmd) (*Node, error) {
	nodeConfig, ok := common.Config.Nodes[name]
	if !ok {
		log.Fatal("config entry does not exist for node", "name", name)
	}

	cmd := createCmd(nodeConfig.Command) 
	cmds = append(cmds, cmd)

	reader, writer, err := commandSetup(cmd)
	if err != nil {
		panic(err)
	}

	n := &Node{
		pendingRequests: xsync.NewMapOf[int, chan *protocol.Response](),
		Capabilities: make(map[string]struct{}),
		reader: reader,
		writer: writer,
		name: name,
	}

	// response reader
	go n.responseReader()

	// initialization
	request := protocol.NewRequest(protocol.InitializeMethod, protocol.InitializeParams{
		BatchSize: common.Config.BatchSize,
		Config: nodeConfig.Config,
	})

	result := new(protocol.InitializeResult)
	err = n.Request(request, result)
	if err != nil {
		return n, err
	}

	if result.Version != protocol.Version {
		return n, fmt.Errorf("node version did not match protocol version: %v, %v\n", result.Version, protocol.Version)
	}

	// capabilities
	for _, method := range result.Capabilities {
		n.Capabilities[method] = struct{}{}
	}

	log.Info("initialized node", "name", name)

	return n, err
}
