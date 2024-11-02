package media

type Media struct {
	Url string `json:"url"`
	Type Type `json:"type"`
	EssentialMetadata `json:"essentialMetadata"`
	TypeMetadata `json:"typeMetadata"`
	NodeMetadata `json:"nodeMetadata"`
}
