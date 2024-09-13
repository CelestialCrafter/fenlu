use fluent_uri::UriRef;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub title: String,
    #[serde(default)]
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
