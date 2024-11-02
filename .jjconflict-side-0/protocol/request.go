package protocol

import (
	"sync/atomic"
)

type Request struct {
	ID int `json:"id"`
	Method string `json:"method"`
	Params interface{} `json:"params"`
}

var id = atomic.Int32{}

func NewRequest(method string, params interface{}) Request {
	return Request {
		ID: int(id.Add(1)),
		Method: method,
		Params: params,
	}
}

