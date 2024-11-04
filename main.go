package main

import (
	"context"
	"os/exec"
	"strings"
	"sync"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/charmbracelet/log"
)

func createCmd(name string) *exec.Cmd {
	command := strings.Split(config.Config.Nodes[name].Command, " ")
	return exec.Command(command[0], command[1:]...)
}

func handleCmd(cmd *exec.Cmd) {
}

type pipeline struct {
	Sources []node.Source
	Sinks []node.Sink
}

func handleNodeErrors(cancel context.CancelFunc, errorChannels ...<-chan error) {
	for _, channel := range errorChannels {
		go func() {
			for err := range channel {
				log.Error("node errored; flushing pipeline", "error", err)
				cancel()
			}
		}()
	}
}

func main() {
	err := config.LoadConfig()
	if err != nil {
		log.Fatal("could not load config", "error", err)
	}
	config.SetupLogger()

	wg := sync.WaitGroup{}
	totalNodes := len(config.Config.Pipeline.Sources) + len(config.Config.Pipeline.Sinks) + len(config.Config.Pipeline.Processors)
	cmds := make([]*exec.Cmd, 0, totalNodes)
	ctx, cancel := context.WithCancel(context.Background())

	sourceMedia, sourceErrors, err := runSources(&wg, cmds, ctx)
	if err != nil {
		log.Fatal("could not initialize sources", "error", err)
	}

	processorMedia, processorErrors, err := runProcessors(&wg, cmds, sourceMedia)

	sinkErrors, err := runSinks(&wg, cmds, processorMedia)
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
}
