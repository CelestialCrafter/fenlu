package main

import (
	"os/exec"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/charmbracelet/log"
)

func runSource(channel chan<- []media.Media, source node.Source) error {
	state := 0
	for {
		media, finished, err := source.Generate(state)
		if err != nil {
			return err
		}

		channel <- media
		state++

		if finished {
			break
		}
	}

	return nil
}

func runSources(wg *sync.WaitGroup, cmds []*exec.Cmd) (<-chan []media.Media, <-chan error, error) {
	sources := config.Config.Pipeline.Sources
	bufferSize := config.Config.BatchSize * len(sources) * 10

	mediaChannel := make(chan []media.Media, bufferSize)
	errorChannel := make(chan error)

	for _, name := range sources {
		cmd := createCmd(name)
		cmds = append(cmds, cmd)

		n, err := node.InitializeNode(cmd, name)
		if err != nil {
			return nil, nil, err
		}
		source := node.Source{Node: n}

		wg.Add(1)
		go func()  {
			defer wg.Done()

			err := runSource(mediaChannel, source)
			if err != nil {
				log.Error(err)
				errorChannel <- err
			}
		}()
	}

	return mediaChannel, errorChannel, nil
}

