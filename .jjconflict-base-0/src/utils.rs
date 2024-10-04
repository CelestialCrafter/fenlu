use std::path::PathBuf;

use uuid::Uuid;

use crate::config::CONFIG;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn path_to_name(path: &PathBuf) -> String {
    path
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .expect("path should be utf-8")
}

pub fn is_script_whitelisted(path: &PathBuf) -> bool {
    let name = path_to_name(path);
    for script in CONFIG.whitelisted_scripts.clone() {
        if name.ends_with(&script) {
            return true
        }
    }

    false
}

