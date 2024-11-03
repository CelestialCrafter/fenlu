package node

import (
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Transform struct {
	Node
}

func (t *Transform) Transform(media []media.Media) ([]media.Media, error) {
	result := new(protocol.TransformResult)
	err := t.Request(
		protocol.NewRequest(
			protocol.TransformMethod,
			protocol.TransformParams(media),
		), 
		result,
	)

	return *result, err
}
