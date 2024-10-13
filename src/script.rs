use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};

use eyre::{OptionExt, Report, Result};
use futures::future::join_all;
use tokio::{
    sync::{mpsc, Mutex},
    task,
    time::sleep,
};
use tracing::{debug, error, info_span, Instrument};

use crate::{
    protocol::{
        capabilities::{self, Capabilities},
        messages::{Id, Request, Response},
    },
    utils::{self, generate_id},
};

#[derive(Debug)]
pub struct Script {
    pub path: PathBuf,
    pub capabilities: Capabilities,
    request_tx: mpsc::Sender<Request>,
    pending_requests: Mutex<HashMap<Id, Option<Response>>>,
}

impl Script {
    pub async fn request(&self, req: Request) -> Response {
        let id = req.id.clone();
        self.request_tx.send(req).await.unwrap();
        sleep(Duration::from_secs(1)).await;
        let mut pending = self.pending_requests.lock().await;
        pending.remove(id.as_str()).unwrap().unwrap()
    }
}

pub async fn spawn_server(path: PathBuf) -> Result<Arc<Script>> {
    let path = path.canonicalize()?;

    let mut dir = path.clone();
    dir.pop();

    let mut child = Command::new(path.clone())
        .current_dir(dir)
        .env("PYTHONUNBUFFERED", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = BufReader::new(child.stdout.take().ok_or_eyre("could not take stdout")?);
    let mut stdin = BufWriter::new(child.stdin.take().ok_or_eyre("could not take stdout")?);

    let (request_tx, mut request_rx) = mpsc::channel(4);
    let mut script = Script {
        path: path.clone(),
        capabilities: Capabilities::default(),
        request_tx,
        pending_requests: Mutex::new(HashMap::new()),
    };

    // manually request capabilities before the script is wrapped in an Arc and therefore immutable
    {
        let request = &Request {
            id: generate_id(),
            method: capabilities::CAPABILITIES_METHOD.to_string(),
            params: serde_json::to_value(capabilities::CapabilitiesRequest {})?,
        };

        let mut encoded = serde_json::to_vec(request)?;
        encoded.push(b'\n');

        stdin.write_all(&encoded)?;
        stdin.flush()?;

        let mut response = String::new();
        stdout.read_line(&mut response)?;
        let response: Response = serde_json::from_str(&response)?;

        assert!(
            request.id == response.id,
            "initial response id was not the same as capabilities request id"
        );
        script.capabilities = serde_json::from_value(response.result()?)?;
    }

    let script = Arc::new(script);

    // take incomming requests and send them to the script
    let request_handle = {
        let name = utils::path_to_name(&path);

        task::spawn(
            async move {
                while let Some(request) = request_rx.recv().await {
                    let mut encoded = serde_json::to_vec(&request)?;
                    encoded.push(b'\n');
                    stdin.write_all(&encoded)?;
                    stdin.flush()?;
                    debug!(id = ?request.id, method = ?request.method, "initiated request");
                }

                Ok::<_, Report>(())
            }
            .instrument(info_span!("response handler", name = name)),
        )
    };

    // take incoming responses
    // remove them from pending requests
    // send them to the pending request tx
    let response_handle = {
        let script = script.clone();
        let name = utils::path_to_name(&path);

        task::spawn(
            async move {
                for line in stdout.lines() {
                    let response: Response = serde_json::from_str(line?.as_str())?;
                    let id = response.id.clone();
                    let mut pending = script.pending_requests.lock().await;

                    pending.insert(id.clone(), Some(response));
                    debug!(id = ?id, "completed request");
                }

                Ok::<_, Report>(())
            }
            .instrument(info_span!("response handler", name = name)),
        )
    };

    task::spawn(async {
        let results = join_all([request_handle, response_handle]).await;
        match (|| -> Result<()> {
            for result in results {
                result??
            }

            Ok(())
        })() {
            Ok(_) => {}
            Err(err) => error!("script errored: {}", err),
        }
    });

    Ok(script)
}
