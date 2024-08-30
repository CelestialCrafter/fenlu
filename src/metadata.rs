use fluent_uri::UriRef;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub uri: UriRef<String>,
    #[serde(default)]
    pub extra_source: String,
    #[serde(default)]
    pub source: String,
    pub mime: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: Option<Extra>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Extra {
    Image { width: u64, height: u64 },
}
