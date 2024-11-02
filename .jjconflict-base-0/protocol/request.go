package protocol

type Request struct {
	Id int `json:"id"`
	Method string `json:"method"`
	Params interface{} `json:"params"`
}

