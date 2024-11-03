package media

type Media struct {
	Url string `json:"url"`
	Type string `json:"type"`
	EssentialMetadata `json:"essentialMetadata"`
	TypeMetadata `json:"typeMetadata"`
	ExtraMetadata `json:"extraMetadata"`
}
