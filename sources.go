package main

import (
	"context"
	"fmt"
	"os/exec"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/charmbracelet/log"
)

func runSource(output chan<- []media.Media, source node.Source, ctx context.Context) error {
	state := 0
	for {
		if ctx.Err() != nil {
			break
		}

		media, finished, err := source.Generate(state)
		if err != nil {
			return err
		}

		output <- media
		state++
		if finished {
			break
		}
	}

	return nil
}

func runSources(wg *sync.WaitGroup, cmds []*exec.Cmd, ctx context.Context) (<-chan []media.Media, <-chan error, error) {
	sources := config.Config.Pipeline.Sources
	bufferSize := config.Config.BufferSize * len(sources)

	output := make(chan []media.Media, bufferSize)
	errors := make(chan error)
	sourceWg := sync.WaitGroup{}

	for _, name := range sources {
		cmd := createCmd(name)
		cmds = append(cmds, cmd)

		n, err := node.InitializeNode(cmd, name)
		if err != nil {
			return nil, nil, err
		}
		_, ok := n.Capabilities()[protocol.SourceMethod]
		if !ok {
			panic(fmt.Sprintln(protocol.SourceMethod, " unsupported on node: ", name))
		}
		source := node.Source{Node: n}

		sourceWg.Add(1)
		go func() {
			defer wg.Done()

			err := runSource(output, source, ctx)
			if err != nil {
				errors <- err
			}
		}()
	}

	wg.Add(1)
	go func() {
		defer close(output)
		defer close(errors)
		defer wg.Done()
		defer log.Info("sources finished")
		sourceWg.Wait()
	}()

	return output, errors, nil
}

