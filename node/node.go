package node

import (
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Node interface {
	Request(protocol.Request, any) error 
	Capabilities() []string
}

