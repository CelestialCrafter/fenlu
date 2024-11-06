package main

import (
	"context"
	"errors"
	"fmt"
	"os/exec"
	"slices"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
)

type nodeWrapper struct {
	nodeType string
	node *node.Node
}

func process(wrapper nodeWrapper, input []media.Media) ([]media.Media, error) {
	switch wrapper.nodeType {
	case protocol.TransformMethod:
		return node.Transform{Node: wrapper.node}.Transform(input)
	case protocol.FilterMethod:
		result, err := node.Filter{Node: wrapper.node}.Filter(input)
		if err != nil {
			return nil, err
		}

		filtered := make([]media.Media, 0, len(input))
		for i, included := range result {
			if included {
				filtered = append(filtered, input[i])
			}
		}
		return slices.Clip(filtered), nil
	}

	return nil, errors.New("unsupported method")
}

func runProcessors(wg *sync.WaitGroup, cmds []*exec.Cmd, ctx context.Context, input <-chan []media.Media) (<-chan []media.Media, <-chan error, error) {
	processors := config.Config.Pipeline.Processors
	bufferSize := config.Config.BufferSize * len(processors)

	nodes := make([]nodeWrapper, len(processors))
	output := make(chan []media.Media, bufferSize)
	errors := make(chan error)

	for i, name := range processors {
		cmd := createCmd(name)
		cmds = append(cmds, cmd)

		n, err := node.InitializeNode(cmd, name)
		if err != nil {
			return nil, nil, err
		}

		capabilities := n.Capabilities()

		var nodeType string
		if _, ok := capabilities[protocol.TransformMethod]; ok {
			nodeType = protocol.TransformMethod
		} else if _, ok := capabilities[protocol.FilterMethod]; ok  {
			nodeType = protocol.FilterMethod
		} else {
			panic(fmt.Sprintf("%s/%s method unsupported on node: %s", protocol.TransformMethod, protocol.FilterMethod, name))
		}

		nodes[i] = nodeWrapper{
			node: n,
			nodeType: nodeType,
		}
	}

	wg.Add(1)
	go func() {
		defer wg.Done()
		defer close(output)
		defer close(errors)
		defer log.Info("processors finished")

		MEDIA: for media := range input {
			if ctx.Err() != nil {
				break
			}

			for _, wrapper := range nodes {
				var err error
				media, err = process(wrapper, media)
				if err != nil {
					errors <- err
					continue MEDIA
				}
			}

			output <- media
		}
	}()

	return output, errors, nil
}

