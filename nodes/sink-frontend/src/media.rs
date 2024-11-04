use std::{collections::HashMap, fmt};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
	pub url: String,
        pub essential_metadata: EssentialMetadata,
        #[serde(flatten)]
        pub type_metadata: TypeMetadata,
        pub extra_metadata: Option<HashMap<String, Value>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EssentialMetadata {
    pub title: String,
    pub creation: i64
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "typeMetadata")]
#[serde(rename_all = "camelCase")]
pub enum TypeMetadata {
    Image { width: u64, height: u64 },
    PDF { author: String, summary: String },
}

impl fmt::Display for TypeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Image { width: _, height: _ } => write!(f, "image"),
            Self::PDF { author: _, summary: _ } => write!(f, "pdf")
        }
    }
}
