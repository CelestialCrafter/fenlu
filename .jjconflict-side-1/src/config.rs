use std::fs::read_to_string;

use once_cell::sync::Lazy;
use serde::Deserialize;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let data = read_to_string(CONFIG_PATH).expect("reading config file should succeed");
    toml::from_str::<Config>(data.as_str()).expect("parsing config should succeed")
});
