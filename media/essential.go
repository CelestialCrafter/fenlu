package media

import "time"

type EssentialMetadata struct {
	Title string `json:"title"`
	Creation time.Time `json:"creation"`
}

