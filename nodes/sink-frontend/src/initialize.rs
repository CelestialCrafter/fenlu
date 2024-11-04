use eyre::Result;

use crate::{config::CONFIG, protocol::{self}};

pub fn handle_initialize(params: protocol::InitializeParams) -> Result<protocol::InitializeResult> {
    CONFIG.set(params.config.unwrap_or_default()).expect("config was already set");
    eprintln!("the initialization is upon us...");

    Ok(protocol::InitializeResult {
        capabilities: vec![protocol::SINK_METHOD.to_string()],
        version: protocol::VERSION.to_string()
    })
}
