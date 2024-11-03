package protocol

type Response struct {
	ID int `json:"id"`
	Result any `json:"result"`
	Error string `json:"error"`
}
