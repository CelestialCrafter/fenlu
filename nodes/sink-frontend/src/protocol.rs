use eyre::{ErrReport, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{config::Config, media::Media};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: usize,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub id: usize,
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

pub const INITIALIZE_METHOD: &str = "initialize/initialize";
pub const SINK_METHOD: &str = "media/sink";
pub const VERSION: &str = "b2a8d343480cbaf075c93fd47033db7a2f020773";

#[derive(Debug, Deserialize, Clone)]
pub struct InitializeParams {
    pub config: Option<Config>
}

#[derive(Debug, Serialize, Clone)]
pub struct InitializeResult {
    pub capabilities: Vec<String>,
    pub version: String
}

pub type SinkParams = Vec<Media>;

#[derive(Debug, Serialize, Clone)]
pub struct SinkResult {}

