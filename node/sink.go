package node

import (
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Sink struct {
	*Node
}

func (s Sink) Sink(media []media.Media) error {
	return s.Request(
		protocol.NewRequest(
			protocol.SinkMethod,
			protocol.SinkParams(media),
		), 
		new(protocol.SinkResult),
	)
}
