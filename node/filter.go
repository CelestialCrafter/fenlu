package node

import (
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Filter struct {
	Node
}

func (t *Filter) Filter(media []media.Media) ([]bool, error) {
	result := new(protocol.FilterResult)
	err := t.Request(
		protocol.NewRequest(
			protocol.FilterMethod,
			protocol.FilterParams(media),
		), 
		result,
	)

	return *result, err
}
