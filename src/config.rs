use std::{fs::read_to_string, path::Path, sync::LazyLock};

use serde::Deserialize;

use crate::pipeline::DB_PATH;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub enum PipelineMode {
    Generate,
    GenerateSave,
    Load,
}

impl Default for PipelineMode {
    fn default() -> Self {
        Self::Load
    }
}

#[derive(Deserialize)]
pub struct Colors {
    pub base: [u8; 3],
    pub surface: [u8; 3],
    pub highlight_medium: [u8; 3],
    pub highlight_high: [u8; 3],
    pub text: [u8; 3],
    pub accent: [u8; 3],
}

#[derive(Deserialize)]
pub struct Interface {
    #[serde(default = "default_thumbnail_size")]
    pub thumbnail_size: u16
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub whitelisted_scripts: Vec<String>,
    #[serde(default = "default_pipeline_mode")]
    pub pipeline_mode: PipelineMode,
    #[serde(default = "default_media_update_interval")]
    pub media_update_interval: u64,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
    pub colors: Colors,
    pub interface: Interface
}

fn default_media_update_interval() -> u64 {
    1000
}

fn default_pipeline_mode() -> PipelineMode {
    if Path::new(DB_PATH).exists() {
        PipelineMode::Load
    } else {
        PipelineMode::GenerateSave
    }
}

fn default_buffer_size() -> usize {
    1024
}


fn default_thumbnail_size() -> u16 {
    296
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let data = read_to_string(CONFIG_PATH).expect("could not read config file");
    toml::from_str::<Config>(data.as_str()).expect("could not parse config file")
});
