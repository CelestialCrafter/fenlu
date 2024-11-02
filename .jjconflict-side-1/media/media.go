package media

import "net/url"

type Media struct {
	Url *url.URL `json:"url"`
	Type Type `json:"type"`
	EssentialMetadata
	TypeMetadata
	NodeMetadata
}
