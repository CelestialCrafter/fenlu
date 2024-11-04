package main

import (
	"fmt"
	"os/exec"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
)

func runSinks(wg *sync.WaitGroup, cmds []*exec.Cmd, input <-chan []media.Media) (chan error, error) {
	sinks := make([]node.Sink, len(config.Config.Pipeline.Sinks))
	errorChannel := make(chan error)

	for i, name := range config.Config.Pipeline.Sinks {
		cmd :=  createCmd(name)
		cmds = append(cmds, cmd)

		n, err := node.InitializeNode(cmd, name)
		if err != nil {
			return nil, err
		}
		_, ok := n.Capabilities()[protocol.SinkMethod]
		if !ok {
			panic(fmt.Sprintln(protocol.SinkMethod, " unsupported on node: ", name))
		}
		sinks[i] = node.Sink{Node: n}

	}

	// why is the whitespace fucking exponential
	wg.Add(1)
	go func() {
		defer wg.Done()
		defer log.Info("sinks finished")

		for media := range input {
			for _, sink := range sinks {
				go func() {
					err := sink.Sink(media)
					if err != nil {
						errorChannel <- err
					}
				}()
			}

			log.Info("finished batch")
		}
	}()

	return errorChannel, nil
}

