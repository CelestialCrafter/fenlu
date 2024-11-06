package main

import (
	"context"
	"flag"
	"os/exec"
	"sync"
	"time"

	"github.com/CelestialCrafter/fenlu/common"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/charmbracelet/log"
)

type pipeline struct {
	Sources []node.Source
	Sinks []node.Sink
}

func handleNodeErrors(cancel context.CancelFunc, errorChannels ...<-chan error) {
	for _, channel := range errorChannels {
		go func() {
			for err := range channel {
				log.Error("node errored; stopping pipeline", "error", err)
				cancel()
			}
		}()
	}
}

func main() {
	flag.Parse()
	err := common.LoadConfig()
	if err != nil {
		log.Fatal("could not load config", "error", err)
	}
	common.SetupLogger()

	wg := sync.WaitGroup{}
	totalNodes := len(common.Config.Pipeline.Sources) + len(common.Config.Pipeline.Sinks) + len(common.Config.Pipeline.Processors)
	cmds := make([]*exec.Cmd, 0, totalNodes)

	ctx, cancel := context.WithCancel(context.Background())

	start := time.Now()

	sourceMedia, sourceErrors, err := runSources(&wg, cmds, ctx)
	if err != nil {
		log.Fatal("could not initialize sources", "error", err)
	}

	processorMedia, processorErrors, err := runProcessors(&wg, cmds, ctx, sourceMedia)

	sinkErrors, err := runSinks(&wg, cmds, ctx, processorMedia)
	if err != nil {
		log.Fatal("could not initialize sinks", "error", err)
	}

	go handleNodeErrors(cancel, sourceErrors, processorErrors, sinkErrors)

	wg.Wait()
	log.Info("cleaning up")
	for _, cmd := range cmds {
		err := cmd.Wait()
		exitErr, ok := err.(*exec.ExitError)
		if err != nil && !(ok && exitErr.ExitCode() == -1) {
			log.Fatal("process errored", "error", err)
		}
	}
	
	log.Info("finished pipeline", "duration", time.Since(start))
}
