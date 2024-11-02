package media

type Type = string
const (
    Image Type = "image"
    PDF = "pdf"
)

type TypeMetadata any

type ImageTypeMetadata struct {
	Width int `json:"width"`
	Height int `json:"height"`
}

type PDFTypeMetadata struct {
	Author string `json:"author"`
	Summary string `json:"summary"`
	Pages int `json:"pages"`
}
