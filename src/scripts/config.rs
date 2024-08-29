use std::{fs, path::PathBuf};

use eyre::{eyre, Result};

pub fn load_config(script_path: PathBuf) -> Result<String> {
    let mut config_path = match script_path.into_os_string().into_string() {
        Ok(path) => Ok(path),
        Err(_) => Err(eyre!("could not parse path into utf-8"))
    }?;

    config_path = config_path.strip_suffix("fnl").unwrap_or(&config_path).to_string();
    config_path  += "toml";

    Ok(fs::read_to_string(config_path).unwrap_or("".to_string()))
}
