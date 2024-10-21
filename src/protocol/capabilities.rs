use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    #[default]
    Source,
    Transform,
    Filter,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Query {
    pub set: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Capabilities {
    pub media: (Type, Option<u64>),
    pub query: Query,
    pub actions: Vec<String>
}

pub const CAPABILITIES_METHOD: &str = "capabilities/capabilities";

#[derive(Debug, Serialize, Clone)]
pub struct CapabilitiesRequest {}
pub type CapabilitiesResponse = Capabilities;
