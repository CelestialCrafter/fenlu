package node

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"time"

	"github.com/CelestialCrafter/fenlu/common"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
	"github.com/go-viper/mapstructure/v2"
	"github.com/puzpuzpuz/xsync/v3"
)

type Node struct {
	writer io.Writer
	reader io.Reader
	name string
	capabilities map[string]struct{}
	pendingRequests *xsync.MapOf[int, chan *protocol.Response]
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

			log.Error("could not decode response", "error", err)
			continue
		}

		if response.Result == nil && response.Error == "" {
			log.Error("request did not have response or error: ", "response", response)
			continue
		}


		channel, ok := n.pendingRequests.Load(response.ID)
		if !ok {
			log.Error("received response with no request pending", "response", response)
			continue
		}

		channel <- response
	}
}

func commandSetup(cmd *exec.Cmd) (io.Reader, io.Writer, error) {
	// pipes & start
	cmd.Stderr = os.Stderr

	reader, err := cmd.StdoutPipe()
	if err != nil {
		return nil, nil, err
	}

	writer, err := cmd.StdinPipe()
	if err != nil {
		return nil, nil, err
	}

	err = cmd.Start()
	if err != nil {
		return nil, nil, err
	}

	return reader, writer, nil
}

func InitializeNode(cmd *exec.Cmd, name string) (*Node, error) {
	reader, writer, err := commandSetup(cmd)
	if err != nil {
		panic(err)
	}

	n := &Node{
		pendingRequests: xsync.NewMapOf[int, chan *protocol.Response](),
		capabilities: make(map[string]struct{}),
		reader: reader,
		writer: writer,
		name: name,
	}

	// response reader
	go n.responseReader()

	// initialization
	request := protocol.NewRequest(protocol.InitializeMethod, protocol.InitializeParams{
		BatchSize: common.Config.BatchSize,
		Config: common.Config.Nodes[name].Config,
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
		n.capabilities[method] = struct{}{}
	}

	log.Info("initialized node", "name", name)

	return n, err
}

func (n *Node) Request(request protocol.Request, value any) error {
	start := time.Now()

	channel := make(chan *protocol.Response, 1)
	n.pendingRequests.Store(request.ID, channel) 

	marshalled, err := json.Marshal(request)
	if err != nil {
		return err
	}

	_, err = n.writer.Write(append(marshalled, '\n'))
	if err != nil {
		return err
	}

	response := <-channel
	n.pendingRequests.Delete(response.ID)

	log.Debug("finished request", "name", n.name, "method", request.Method, "id", request.ID, "duration", time.Since(start))

	if response.Error != "" {
		return fmt.Errorf("request %d errored: %w", request.ID, errors.New(response.Error))
	}

	err = mapstructure.Decode(response.Result, value)
	if err != nil {
		return fmt.Errorf("could not decode request: %w", err)
	}


	return nil
}

func (n *Node) Capabilities() map[string]struct{} {
	return n.capabilities
}
