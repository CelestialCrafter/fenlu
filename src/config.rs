use std::{fs::read_to_string, path::Path};

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::pipeline::DB_PATH;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub enum PipelineMode {
    Generate,
    GenerateSave,
    Load
}

impl Default for PipelineMode {
    fn default() -> Self {
        Self::Load
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_media_update_interval")]
    pub media_update_interval: u128,
    #[serde(default)]
    pub whitelisted_scripts: Vec<String>,
    #[serde(default = "default_pipeline_mode")]
    pub pipeline_mode: PipelineMode,
}

fn default_media_update_interval() -> u128 {
    250
}

fn default_pipeline_mode() -> PipelineMode {
    if Path::new(DB_PATH).exists() {
        PipelineMode::Load
    } else {
        PipelineMode::GenerateSave
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let data = read_to_string(CONFIG_PATH).expect("could not read config file");
    toml::from_str::<Config>(data.as_str()).expect("could not parse config file")
});
