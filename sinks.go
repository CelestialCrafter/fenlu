package main

import (
	"context"
	"fmt"
	"os/exec"
	"sync"

	"github.com/CelestialCrafter/fenlu/common"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
)

func runSinks(wg *sync.WaitGroup, cmds []*exec.Cmd, ctx context.Context, input <-chan []media.Media) (<-chan error, error) {
	sinks := make([]node.Sink, len(common.Config.Pipeline.Sinks))

	sinkWg := sync.WaitGroup{}
	errors := make(chan error)

	for i, name := range common.Config.Pipeline.Sinks {
		n, err := node.InitializeNode(name, cmds)
		if err != nil {
			return nil, err
		}
		_, ok := n.Capabilities[protocol.SinkMethod]
		if !ok {
			panic(fmt.Sprintln(protocol.SinkMethod, " unsupported on node: ", name))
		}
		sinks[i] = node.Sink{Node: n}

	}

	wg.Add(1)
	go func() {
		defer close(errors)
		defer wg.Done()
		defer log.Info("sinks finished")

		for media := range input {
			if ctx.Err() != nil {
				break
			}

			for _, sink := range sinks {
				sinkWg.Add(1)
				go func() {
					defer sinkWg.Done()

					err := sink.Sink(media)
					if err != nil {
						errors <- err
					}
				}()
			}
		}

		sinkWg.Wait()
	}()

	return errors, nil
}

