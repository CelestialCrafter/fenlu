use fluent_uri::UriRef;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub uri: UriRef<String>,
    #[serde(default)]
    pub extra_source: String,
    pub mime: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Extra {
    Image { width: u64, height: u64 },
}
