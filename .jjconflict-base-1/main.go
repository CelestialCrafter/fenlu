package main

import (
	"fmt"
	"io"
	"os"
	"os/exec"
	"time"

	"github.com/CelestialCrafter/fenlu/node"
	"github.com/CelestialCrafter/fenlu/protocol"
)

func main() {
	cmd := exec.Command("python", "nodes/source-directory.py")
	cmd.Env = append(cmd.Env, "PYTHONUNBUFFERED=1")
	cmd.Stderr = os.Stderr

	in, err := cmd.StdinPipe()
	if err != nil {
		panic(err)
	}

	var out io.Reader
	out, err = cmd.StdoutPipe()
	if err != nil {
		panic(err)
	}

	err = cmd.Start()
	if err != nil {
		panic(err)
	}

	node, err := node.InitializeNode(in, out)
	if err != nil {
		panic(err)
	}

	go func()  {
		time.Sleep(time.Second * 2)
		cmd.Process.Kill()
	}()

	fmt.Println(node.Capabilities())

	result := new(protocol.SourceResult)
	err = node.Request(
		protocol.NewRequest(
			protocol.SourceMethod,
			protocol.SourceParams{
				State: 0,
			},
		), 
		result,
	)
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v", result)

	err = cmd.Wait()
	if err != nil {
		panic(err)
	}
}
