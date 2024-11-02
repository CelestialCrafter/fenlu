package main

import (
	"fmt"
	"io"
	"os"
	"os/exec"
	"time"

	"github.com/CelestialCrafter/fenlu/node"
)

func main() {
	cmd := exec.Command("python", "test-server.py")
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

	myNode, err := node.InitializeNode(in, out)
	if err != nil {
		panic(err)
	}

	go func()  {
		time.Sleep(time.Second * 2)
		cmd.Process.Kill()
	}()

	fmt.Println(myNode.Capabilities())

	err = cmd.Wait()
	if err != nil {
		panic(err)
	}
}
