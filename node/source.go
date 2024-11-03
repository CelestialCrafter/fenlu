package node

import (
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/protocol"
)

type Source struct {
	*Node
}

func (s Source) Generate(state int) ([]media.Media, bool, error) {
	result := new(protocol.SourceResult)
	err := s.Request(
		protocol.NewRequest(
			protocol.SourceMethod,
			protocol.SourceParams{
				State: state,
			},
		), 
		result,
	)

	return result.Media, result.Finished, err
}
