package protocol

import "github.com/CelestialCrafter/fenlu/media"

const SourceMethod = "media/source"
type SourceParams struct {
	State int `json:"state"`
}
type SourceResult struct {
	Media []media.Media `json:"media"`
	Finished bool `json:"finished"`
}
