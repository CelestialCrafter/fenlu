package node

import (
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
	"github.com/go-viper/mapstructure/v2"
)

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

