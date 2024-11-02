package protocol

type Response struct {
	Id int `json:"id"`
	Result interface{} `json:"result"`
	Error error `json:"error"`
}
