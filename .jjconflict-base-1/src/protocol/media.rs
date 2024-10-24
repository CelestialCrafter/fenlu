use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub history: HashMap<String, Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: Option<Extra>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Extra {
    Image { width: u64, height: u64 },
    PDF { author: String, summary: String },
}

pub const FILTER_METHOD: &str = "media/filter";
pub const TRANSFORM_METHOD: &str = "media/transform";
pub const GENERATE_METHOD: &str = "media/generate";

pub type TransformRequest = Vec<Media>;

#[derive(Debug, Deserialize, Clone)]
pub struct TransformResponse {
    pub media: Vec<Media>,
    #[serde(default)]
    pub extra: Vec<Value>,
}

pub type FilterRequest = Vec<Media>;

#[derive(Debug, Deserialize, Clone)]
pub struct FilterResponse {
    pub included: Vec<bool>,
    #[serde(default)]
    pub extra: Vec<Value>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GenerateRequest {
    pub batch_size: usize,
    pub state: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GenerateResponse {
    pub media: Vec<Media>,
    #[serde(default)]
    pub extra: Vec<Value>,
    pub finished: bool,
}
