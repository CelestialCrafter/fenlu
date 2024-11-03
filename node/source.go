package node

import (
	"github.com/CelestialCrafter/fenlu/media"
	"github.com/CelestialCrafter/fenlu/protocol"
	"github.com/go-viper/mapstructure/v2"
)

type Source struct {
	Node
}

func (s *Source) Generate(state int) ([]media.Media, bool, error) {
	result := new(protocol.SourceResult)
	err := s.Request(
		protocol.NewRequest(
			protocol.SourceMethod,
			protocol.SourceParams{
				State: 0,
			},
		), 
		result,
	)

	// @TODO remove this? along with the types. they shouldnt be used anywhere in the core codebase
	newMedia := make([]media.Media, len(result.Media))
	for i, m := range result.Media {
		originalTypeMetadata := m.TypeMetadata
		switch m.Type {
		case media.Image:
			m.TypeMetadata = media.ImageTypeMetadata{}
		case media.PDF:
			m.TypeMetadata = media.PDFTypeMetadata{}
		}

		mapstructure.Decode(originalTypeMetadata, &m.TypeMetadata)
		newMedia[i] = m
	}

	return newMedia, result.Finished, err
}
