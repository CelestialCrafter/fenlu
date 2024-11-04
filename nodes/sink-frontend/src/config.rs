use std::sync::OnceLock;

use serde::Deserialize;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {}
