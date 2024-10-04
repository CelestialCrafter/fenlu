use std::{io::{BufRead, BufReader, BufWriter, Write}, path::PathBuf, process::{Command, Stdio}, sync::Arc};

use dashmap::DashMap;
use eyre::{eyre, OptionExt, Report, Result};
use futures::future::join_all;
use tokio::{sync::{mpsc, oneshot}, task};

use crate::{protocol::{capabilities::{self, Capabilities}, messages::{Id, Request, Response}}, utils::generate_id};

#[derive(Debug)]
pub struct Script {
    pub path: PathBuf,
    pub capabilities: Capabilities,
    request_tx: mpsc::Sender<Request>,
    pending_requests: DashMap<Id, oneshot::Sender<Response>>
}

impl Script {
    pub async fn request(&self, req: Request) -> Response {
        let (tx, rx) = oneshot::channel();
        if let Some(_) = self.pending_requests.insert(req.id.clone(), tx) {
            panic!("request was already in pending map");
        }

        self.request_tx.send(req).await.expect("reciever was dropped");
        rx.await.expect("sender was dropped")
    }
}

pub async fn spawn_server(path: PathBuf) -> Result<Arc<Script>> {
    let path = path.canonicalize()?;

    let mut dir = path.clone();
    dir.pop();

    let mut child = Command::new(path.clone())
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = BufReader::new(child.stdout.take().ok_or_eyre("could not take stdout")?);
    let mut stdin = BufWriter::new(child.stdin.take().ok_or_eyre("could not take stdout")?);

    let (request_tx, mut request_rx) = mpsc::channel(4);
    let mut script = Script {
        path,
        capabilities: Capabilities::default(),
        request_tx,
        pending_requests: DashMap::new()
    };

    // manually request capabilities before the script is wrapped in an Arc and therefore immutable
    {
        let id = generate_id();

        let mut encoded = serde_json::to_vec(&Request {
            id: id.clone(),
            method: capabilities::CAPABILITIES_METHOD.to_string(),
            params: serde_json::to_value(capabilities::CapabilitiesRequest {})?
        })?;
        encoded.push(b'\n');

        stdin.write_all(&encoded)?;
        stdin.flush()?;

        let mut response = String::new();
        stdout.read_line(&mut response)?;
        let response: Response = serde_json::from_str(&response)?;

        assert!(id == response.id, "initial response id was not the same as capabilities request id");
        script.capabilities = serde_json::from_value(response.result()?)?;
    }

    let script = Arc::new(script);

    // take incomming requests and send them to the script
    let request_handle = task::spawn(async move {
        while let Some(request) = request_rx.recv().await {
            let mut encoded = serde_json::to_vec(&request)?;
            encoded.push(b'\n');
            stdin.write_all(&encoded)?;
            stdin.flush()?;
            println!("-> {}", request.id);
        }

        Ok::<_, Report>(())
    });

    // take incomming responses
    // remove them from pending requests
    // send them to the pending request tx
    let response_handle = {
        let script = script.clone();
        task::spawn(async move {
            for line in stdout.lines() {
                let response: Response = serde_json::from_str(line?.as_str())?;
                let id = response.id.clone();
                if let Some((_, tx)) = script.pending_requests.remove(&response.id) {
                    tx.send(response).map_err(|_| eyre!("receiver was dropped"))?;
                    println!("<- {}", id);
                }
            }

            Ok::<_, Report>(())
        })
    };

    task::spawn(async {
        let results = join_all([request_handle, response_handle]).await;
        match (|| -> Result<()> {
            for result in results {
                result??
            }

            Ok(())
        })() {
            Ok(_) => {},
            Err(err) => eprintln!("script errored: {}", err)
        }
    });

    Ok(script)
}

