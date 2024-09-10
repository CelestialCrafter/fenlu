use eyre::{eyre, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn load_config(script_path: PathBuf) -> Result<String> {
    let mut config_path = match script_path.into_os_string().into_string() {
        Ok(path) => Ok(path),
        Err(_) => Err(eyre!("could not parse path into utf-8")),
    }?;

    config_path = config_path
        .strip_suffix("fnl")
        .unwrap_or(&config_path)
        .to_string();
    config_path += "toml";

    Ok(fs::read_to_string(config_path).unwrap_or("".to_string()))
}

pub fn compile_fennel(path: PathBuf) -> (String, String) {
    let output = Command::new("fennel")
        .arg("-c")
        .arg(path.clone())
        .output()
        .expect("compilation should succeed");
    let compiled = String::from_utf8(output.stdout).expect("path should be utf-8");

    let config = load_config(path).expect("config should load");

    (compiled, config)
}
