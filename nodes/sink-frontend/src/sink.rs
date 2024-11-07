use std::sync::mpsc::Sender;

use eyre::Result;
use crate::{media::Media, protocol};

pub fn handle_sink(params: protocol::SinkParams, tx: Sender<Vec<Media>>) -> Result<protocol::SinkResult> {
    tx.send(params).expect("could not send media");
    return Ok(protocol::SinkResult {})
}
