package main

import (
	"os/exec"
	"strings"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/node"
)

func createCmd(name string) *exec.Cmd {
	command := strings.Split(config.Config.Nodes[name].Command, " ")
	return exec.Command(command[0], command[1:]...)
}

func main() {
	err := config.LoadConfig()
	if err != nil {
		panic(err)
	}

	// setup
	name := "source-pixiv"
	n, err := node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	source := node.Source{Node: n}

	name = "sink-print"
	n, err = node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	sink := node.Sink{Node: n}

	name = "transform-proxy"
	n, err = node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	transform := node.Transform{Node: n}

	media, _, err := source.Generate(0)
	if err != nil {
		panic(err)
	}

	media, err = transform.Transform(media)
	if err != nil {
		panic(err)
	}

	err = sink.Sink(media)
	if err != nil {
		panic(err)
	}
}
