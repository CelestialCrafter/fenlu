package main

import (
	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/node"
)

func runSinks(mediaChannel <-chan []media.Media) (chan error, error) {
	errorChannel := make(chan error)

	for _, name := range config.Config.Pipeline.Sinks {
		n, err := node.InitializeNode(createCmd(name), name)
		if err != nil {
			return nil, err
		}
		sink := node.Sink{Node: n}

		for media := range mediaChannel {
			go func() {
				if sink.Sink(media) != nil {
					errorChannel <- err
				}
			}()
		}
	}

	return errorChannel, nil
}

