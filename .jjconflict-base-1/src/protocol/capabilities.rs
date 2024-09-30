use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Media {
    pub source: bool,
    pub transform: bool,
    pub filter: bool
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Query {
    pub query: bool,
    pub completion: bool
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Capabilities {
    pub media: Media,
    pub query: Query,
}
