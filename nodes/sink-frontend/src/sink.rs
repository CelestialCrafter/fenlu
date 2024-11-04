use eyre::Result;
use crate::protocol;

pub fn handle_sink(params: protocol::SinkParams) -> Result<protocol::SinkResult> {
    for media in params {
        //eprintln!("{:?}", media);
    }

    return Ok(protocol::SinkResult {})
}
