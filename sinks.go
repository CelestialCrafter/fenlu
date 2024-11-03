package main

import (
	"os/exec"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
)

func runSinks(wg *sync.WaitGroup, cmds []*exec.Cmd, mediaChannel <-chan []media.Media) (chan error, error) {
	sinks := config.Config.Pipeline.Sinks
	errorChannel := make(chan error)

	for _, name := range sinks {
		cmd :=  createCmd(name)
		cmds = append(cmds, cmd)

		n, err := node.InitializeNode(cmd, name)
		if err != nil {
			return nil, err
		}
		sink := node.Sink{Node: n}

		go func()  {
			for media := range mediaChannel {
				go func() {
					err := sink.Sink(media)
					if err != nil {
						errorChannel <- err
					}
				}()
			}

		}()
	}

	return errorChannel, nil
}

