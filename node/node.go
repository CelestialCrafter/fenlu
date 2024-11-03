package node

import (
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Node interface {
	// @TODO add context to request
	Request(protocol.Request, any) error 
	Capabilities() map[string]struct{}
}

