use std::sync::mpsc::Sender;

use eyre::Result;
use crate::{media::Media, protocol};

pub fn handle_sink(params: protocol::SinkParams, tx: Sender<Media>) -> Result<protocol::SinkResult> {
    for media in params {
        tx.send(media).expect("could not send media");
    }

    return Ok(protocol::SinkResult {})
}
