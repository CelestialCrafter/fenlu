package protocol

import (
	"sync/atomic"
)

type Request struct {
	ID int `json:"id"`
	Method string `json:"method"`
	Params any `json:"params"`
}

var id = atomic.Int32{}

func NewRequest(method string, params any) Request {
	return Request {
		ID: int(id.Add(1)),
		Method: method,
		Params: params,
	}
}

