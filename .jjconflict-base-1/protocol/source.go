package protocol

import "github.com/CelestialCrafter/fenlu/media"

type SourceParams struct {
	State int
}
type SourceResult struct {
	Media []media.Media
	Finished bool
}
