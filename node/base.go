package node

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"

	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/go-viper/mapstructure/v2"
)

type BaseNode struct {
	in io.Writer
	capabilities []string
	pendingRequests map[int]chan *protocol.Response
}

func responseReader(n *BaseNode, input io.Reader) {
	decoder := json.NewDecoder(input)
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

func InitializeNode(in io.Writer, out io.Reader) (Node, error) {
	n := &BaseNode{
		pendingRequests: make(map[int]chan *protocol.Response),
		in: in,
	}

	// response reader
	go responseReader(n, out)

	// initialization
	// @TODO fill in params
	request := protocol.NewRequest(protocol.InitializeMethod, protocol.InitializeParams{})

	result := new(protocol.InitializeResult)
	err := n.Request(request, result)
	if err != nil {
		return n, err
	}

	if result.Version != protocol.Version {
		panic(fmt.Sprintf("node version did not match protocol version: %v, %v\n", result.Version, protocol.Version))
	}

	n.capabilities = result.Capabilities

	return n, err
}

func (n *BaseNode) Request(request protocol.Request, value any) error {
	n.pendingRequests[request.ID] = make(chan *protocol.Response, 1)

	marshalled, err := json.Marshal(request)
	if err != nil {
		return err
	}

	_, err = n.in.Write(append(marshalled, '\n'))
	if err != nil {
		return err
	}

	response := <-n.pendingRequests[request.ID]
	err = mapstructure.Decode(response.Result, value)
	if err != nil {
		return err
	}

	if response.Error != "" {
		return errors.New(response.Error)
	}

	return nil
}

func (n *BaseNode) Capabilities() []string {
	return n.capabilities
}
