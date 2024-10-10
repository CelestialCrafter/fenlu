use fluent_uri::UriRef;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub title: String,
    pub uri: UriRef<String>,
    #[serde(default)]
    pub extra_source: String,
    #[serde(default)]
    pub history: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: Option<Extra>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Extra {
    Image { width: u64, height: u64 },
    PDF { author: String, summary: String }
}

pub const FILTER_METHOD: &str = "media/filter";
pub const TRANSFORM_METHOD: &str = "media/transform";
pub const GENERATE_METHOD: &str = "media/generate";

pub type TransformRequest = Media;
pub type TransformResponse = Media;

pub type FilterRequest = Media;
#[derive(Debug, Deserialize, Clone)]
pub struct FilterResponse {
    pub included: bool
}

#[derive(Debug, Serialize, Clone)]
pub struct GenerateRequest {
    pub batch_size: u32
}
#[derive(Debug, Deserialize, Clone)]
pub struct GenerateResponse {
    pub media: Vec<Media>,
    pub finished: bool
}

