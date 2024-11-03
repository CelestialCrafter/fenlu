package main

import (
	"os/exec"
	"strings"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/node"
	"github.com/charmbracelet/log"
)

func createCmd(name string) *exec.Cmd {
	command := strings.Split(config.Config.Nodes[name].Command, " ")
	return exec.Command(command[0], command[1:]...)
}

func handleCmd(cmd *exec.Cmd) {
	err := cmd.Wait()
	exitErr, ok := err.(*exec.ExitError)
	if err != nil && !(ok && exitErr.ExitCode() == -1) {
		log.Fatal("process errored", "error", err)
	}
}

type pipeline struct {
	Sources []node.Source
	Sinks []node.Sink
}

func handleNodeErrors(errorChannel <-chan error) {
	for err := range errorChannel {
		log.Error("node errored", "error", err)
	}
}

func main() {
	err := config.LoadConfig()
	if err != nil {
		log.Fatal("could not load config", "error", err)
	}
	config.SetupLogger()

	// sources
	sourceMedia, errorChannel, err := runSources()
	if err != nil {
		log.Fatal("could not initialize sources", "error", err)
	}

	go handleNodeErrors(errorChannel)

	// sinks
	errorChannel, err = runSinks(sourceMedia)
	if err != nil {
		log.Fatal("could not initialize sinks", "error", err)
	}
	go handleNodeErrors(errorChannel)
}
