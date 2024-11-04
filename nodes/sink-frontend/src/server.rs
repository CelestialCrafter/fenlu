use std::{io::{self, Write}, sync::mpsc::Sender};

use eyre::{eyre, Result};
use serde_json::Value;

use crate::{initialize, media::Media, protocol, sink};

fn get_result(method: &str, params: Value, tx: Sender<Media>) -> Result<Value> {
    Ok(match method {
        protocol::INITIALIZE_METHOD => serde_json::to_value(initialize::handle_initialize(serde_json::from_value(params)?)?)?,
        protocol::SINK_METHOD => serde_json::to_value(sink::handle_sink(serde_json::from_value(params)?, tx)?)?,
        _ => return Err(eyre!("unsupported method"))
    })
}

pub fn listen(tx: Sender<Media>) {
    let mut stdout = io::stdout();
    let lines = io::stdin().lines();

    for line in lines {
        let request: protocol::Request = serde_json::from_str(&line.expect("could not read line")).expect("could not decode json");

        let result = get_result(request.method.as_str(), request.params, tx.clone());
        let response = protocol::Response {
            id: request.id,
            result: match result {
                Ok(ref result) => Some(result.clone()),
                Err(_) => None
            },
            error: match result {
                Ok(_) => None,
                Err(error) => Some(error.to_string())
            },
        };

        let buf = serde_json::to_vec(&response).expect("could not encode to json");
        stdout.write_all(buf.as_slice()).expect("could not write to stdout");
        stdout.flush().expect("could not flush stdout");
    }
}
