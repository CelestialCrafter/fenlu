use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Media {
    pub source: (bool, Option<u64>),
    pub transform: (bool, Option<u64>),
    pub filter: (bool, Option<u64>),
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Query {
    pub set: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Capabilities {
    pub media: Media,
    pub query: Query,
}

pub const CAPABILITIES_METHOD: &str = "capabilities/capabilities";

#[derive(Debug, Serialize, Clone)]
pub struct CapabilitiesRequest {}
pub type CapabilitiesResponse = Capabilities;
