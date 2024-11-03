package main

import (
	"os/exec"
	"slices"
	"strings"

	"github.com/CelestialCrafter/fenlu/config"
	"github.com/CelestialCrafter/fenlu/media"
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

	name = "filter-tags"
	n, err = node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	filter := node.Filter{Node: n}

	name = "transform-proxy"
	n, err = node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	transform := node.Transform{Node: n}

	name = "sink-print"
	n, err = node.InitializeNode(createCmd(name), name)
	if err != nil {
		panic(err)
	}
	sink := node.Sink{Node: n}

	// source
	sourced, _, err := source.Generate(0)
	if err != nil {
		panic(err)
	}

	// filter
	filtered := make([]media.Media, 0, len(sourced))
	filterResult, err := filter.Filter(sourced)
	if err != nil {
		panic(err)
	}

	for i, included := range filterResult {
		if included {
			filtered = append(filtered, sourced[i])
		}
	}
	filtered = slices.Clip(filtered)

	// transform
	transformed, err := transform.Transform(filtered)
	if err != nil {
		panic(err)
	}

	// sink
	err = sink.Sink(transformed)
	if err != nil {
		panic(err)
	}
}
