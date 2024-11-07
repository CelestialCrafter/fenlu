use std::sync::OnceLock;

use serde::Deserialize;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Action {
    pub name: String,
    pub command: String
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub actions: Vec<Action>,
    pub render_amount: usize
}
