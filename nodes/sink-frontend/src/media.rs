use std::{collections::HashMap, fmt};

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
	pub url: String,
        pub essential_metadata: EssentialMetadata,
        #[serde(flatten)]
        pub type_metadata: TypeMetadata,
        pub extra_metadata: Option<HashMap<String, Value>>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
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

impl Default for TypeMetadata {
    fn default() -> Self {
        Self::Image {
            width: u64::default(),
            height: u64::default()
        }
    }
}

impl fmt::Display for TypeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Image { width: _, height: _ } => write!(f, "image"),
            Self::PDF { author: _, summary: _ } => write!(f, "pdf")
        }
    }
}
