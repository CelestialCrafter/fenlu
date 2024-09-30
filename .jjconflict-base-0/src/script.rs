use std::{io::{BufRead, BufReader, BufWriter, Write}, path::PathBuf, process::{Command, Stdio}, sync::Arc};

use dashmap::DashMap;
use eyre::{eyre, OptionExt, Report, Result};
use futures::future::join_all;
use tokio::{sync::{mpsc, oneshot}, task};

use crate::{protocol::{capabilities::Capabilities, media::Media, messages::{Id, Request, Response}}, utils::{self, child_guard::ChildGuard}};

pub struct Script {
    pub path: PathBuf,
    pub capabilities: Capabilities,
    request_tx: mpsc::Sender<Request>,
    pending_requests: DashMap<Id, oneshot::Sender<Response>>
}

impl Script {
    pub async fn request(&self, req: Request) -> Result<Response> {
        let (tx, rx) = oneshot::channel();
        if let Some(_) = self.pending_requests.insert(req.id, tx) {
            panic!("request was already in pending map");
        }

        rx.await.map_err(|_| eyre!("receiver was dropped"))
    }
}

pub fn spawn_server(path: PathBuf) -> Result<Arc<Script>> {
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

    let _guard = ChildGuard(child);

    let mut capabilities = String::new();
    stdout.read_line(&mut capabilities)?;
    let capabilities = serde_json::from_str(capabilities.as_str())?;

    let bs = 4;
    let (request_tx, mut request_rx) = mpsc::channel(bs);
    let script = Arc::new(Script {
        path,
        capabilities,
        request_tx,
        pending_requests: DashMap::new()
    });

    let request_handle = task::spawn(async move {
        while let Some(request) = request_rx.recv().await {
            let encoded = serde_json::to_vec(&request)?;
            stdin.write_all(&encoded)?;
        }

        Ok::<_, Report>(())
    });

    let response_handle = {
        let script = script.clone();
        task::spawn(async move {
            for line in stdout.lines() {
                let response: Response = serde_json::from_str(line?.as_str())?;
                if let Some((_, tx)) = script.pending_requests.remove(&response.id) {
                    tx.send(response).map_err(|_| eyre!("receiver was dropped"))?;
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

pub fn append_history(path: PathBuf, mut media: Media) {
    let name = utils::path_to_name(&path);
    media.history.push(name);
}

