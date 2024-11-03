package node

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/go-viper/mapstructure/v2"
)

type BaseNode struct {
	writer io.Writer
	reader io.Reader
	capabilities map[string]struct{}
	pendingRequests map[int]chan *protocol.Response
}

func (n *BaseNode) responseReader() {
	decoder := json.NewDecoder(n.reader)
	for {
		response := new(protocol.Response)

		err := decoder.Decode(response)
		if err != nil {
			if errors.Is(err, io.EOF) {
				break
			}

			panic(fmt.Sprintln("could not decode response: ", err))
		}

		if n.pendingRequests[response.ID] == nil {
			panic(fmt.Sprintln("received response with no request pending: ", response))
		}


		if response.Result == nil && response.Error == "" {
			panic(fmt.Sprintln("request did not have response or error: ", response.ID))
		}

		n.pendingRequests[response.ID] <- response
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

func InitializeNode(cmd *exec.Cmd, name string) (Node, error) {
	reader, writer, err := commandSetup(cmd)
	if err != nil {
		panic(err)
	}

	n := &BaseNode{
		pendingRequests: make(map[int]chan *protocol.Response),
		capabilities: make(map[string]struct{}),
		reader: reader,
		writer: writer,
	}

	// response reader
	go n.responseReader()

	// initialization
	// @TODO fill in params
	request := protocol.NewRequest(protocol.InitializeMethod, protocol.InitializeParams{
		BatchSize: config.Config.BatchSize,
		Config: config.Config.Nodes[name].Config,
	})

	result := new(protocol.InitializeResult)
	err = n.Request(request, result)
	if err != nil {
		return n, err
	}

	if result.Version != protocol.Version {
		panic(fmt.Sprintf("node version did not match protocol version: %v, %v\n", result.Version, protocol.Version))
	}

	// capabilities
	for _, method := range result.Capabilities {
		n.capabilities[method] = struct{}{}
	}

	return n, err
}

func (n *BaseNode) Request(request protocol.Request, value any) error {
	n.pendingRequests[request.ID] = make(chan *protocol.Response, 1)

	marshalled, err := json.Marshal(request)
	if err != nil {
		return err
	}

	_, err = n.writer.Write(append(marshalled, '\n'))
	if err != nil {
		return err
	}

	response := <-n.pendingRequests[request.ID]
	delete(n.pendingRequests, request.ID)

	err = mapstructure.Decode(response.Result, value)
	if err != nil {
		return err
	}

	if response.Error != "" {
		return errors.New(response.Error)
	}

	return nil
}

func (n *BaseNode) Capabilities() map[string]struct{} {
	return n.capabilities
}
