use eyre::{ErrReport, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Id = String;
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Id,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: Id,
    pub result: Option<Value>,
    pub error: Option<String>
}

impl Response {
    pub fn result(&self) -> Result<Value> {
        match self.error.clone() {
            Some(message) => Err(ErrReport::msg(message)),
            None => Ok(self.result.clone().expect("result should exist if error does not"))
        }
    }
}
