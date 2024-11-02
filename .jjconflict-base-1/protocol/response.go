package protocol

type Response struct {
	ID int `json:"id"`
	Result interface{} `json:"result"`
	Error error `json:"error"`
}
