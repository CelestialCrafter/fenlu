use std::{path::PathBuf, process::Command};

use eyre::Result;

pub fn compile_fennel(path: PathBuf) -> Result<String> {
    let output = Command::new("fennel").arg("-c").arg(path).output()?;

    let compiled = String::from_utf8(output.stdout)?;
    Ok(compiled)
}
