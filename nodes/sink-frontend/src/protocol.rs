use std::collections::HashMap;

use eyre::{ErrReport, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;

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
pub const VERSION: &str = "95a247050de65c132541eabe3d93ca0b7c9b5a65";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
	pub url: String,
        pub essential_metadata: EssentialMetadata,
        #[serde(flatten)]
        pub type_metadata: TypeMetadata,
        pub extra_metadata: Option<HashMap<String, Value>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EssentialMetadata {
    pub title: String,
    pub creation: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "typeMetadata")]
#[serde(rename_all = "camelCase")]
pub enum TypeMetadata {
    Image { width: u64, height: u64 },
    PDF { author: String, summary: String },
}

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

