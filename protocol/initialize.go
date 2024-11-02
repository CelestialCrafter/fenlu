package protocol

const InitializeMethod = "initialize/initialize"
type InitializeParams struct {
	BatchSize int `json:"batchSize"`
	Config interface{} `json:"config"`
}

type InitializeResult struct {
	Capabilities []string `json:"capabilities"`
	Version string `json:"version"`
}
