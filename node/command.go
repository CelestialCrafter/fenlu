package node

import (
	"io"
	"os"
	"os/exec"
	"runtime"
)

func createCmd(command string) *exec.Cmd {
	var shell string
	var flag string
	if runtime.GOOS == "windows" {
		shell = "cmd"
		flag = "/c"
	} else {
		shell = "sh"
		flag = "-c"
	}

	return exec.Command(shell, flag, command)
}

func commandSetup(cmd *exec.Cmd) (io.Reader, io.Writer, error) {
	// pipes & start
	cmd.Stderr = os.Stderr

	reader, err := cmd.StdoutPipe()
	if err != nil {
		return nil, nil, err
	}

	writer, err := cmd.StdinPipe()
	if err != nil {
		return nil, nil, err
	}

	err = cmd.Start()
	if err != nil {
		return nil, nil, err
	}

	return reader, writer, nil
}

