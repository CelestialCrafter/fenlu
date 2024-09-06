use std::fs::read_to_string;

use once_cell::sync::Lazy;
use serde::Deserialize;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_generation_script")]
    pub generation_script: String,
    #[serde(default = "default_load_script")]
    pub load_script: String
}

fn default_generation_script() -> String {
    "fenlu-cli -m save scripts/*".to_string()
}

fn default_load_script() -> String {
    "fenlu-cli -m load scripts/*".to_string()
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let data = read_to_string(CONFIG_PATH).expect("reading config file should not fail");
    toml::from_str::<Config>(data.as_str()).expect("parsing config should not fail")
});
