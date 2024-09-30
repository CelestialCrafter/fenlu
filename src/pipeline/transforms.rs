use std::{io::{BufRead, BufReader, Write}, path::PathBuf, process::{Command, Stdio}, sync::mpsc::{channel, Receiver}};

use eyre::Result;
use tokio::task;
use crate::{protocol::media::Media, utils};

use super::Queries;

fn create_transform(path: PathBuf, input: Receiver<Media>, query: String) -> Receiver<Media> {
    let (tx, rx) = channel();

    let path = path.canonicalize().expect("path should be canonicalizable");

    let mut dir = path.clone();
    dir.pop();

    let mut child = Command::new(path.clone())
        .arg(query)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("could not start command");

    let stdout = child.stdout.take().expect("could not take stdout");
    let mut stdin = child.stdin.take().expect("could not take stdin");
    let reader = BufReader::new(stdout);

    task::spawn(async move {
        for media in input.iter() {
            let string = serde_json::to_string(&media).expect("Metadata should decode to string") + "\n";
            stdin.write_all(string.as_bytes()).expect("should be able to write stdin");
        }
    });

    task::spawn(async move {
        let name = utils::path_to_name(&path);

        for line in reader.lines() {
            let line = line.expect("should be able to read line");
            let mut media: Media = serde_json::from_str(line.as_str()).expect("line should decode to Metadata");
            media.history.push(name.clone());
            tx.send(media).expect("reciever should not drop");
        }
    });
    
    rx
}

pub fn scripts() -> impl Iterator<Item = std::result::Result<PathBuf, GlobError>> {
    glob("scripts/transform-*").expect("glob should be valid")
}

pub async fn apply_transforms(
    input: Receiver<Media>,
    queries: Queries
) -> Result<Receiver<Media>> {
    let (tx, rx) = channel();

    let data: Vec<(PathBuf, String)> = scripts()
        .map(|path| path.expect("could not read path"))
        .filter(|path| utils::is_script_whitelisted(path))
        .map(|path| (path.clone(), queries.get(&utils::path_to_name(&path)).cloned().unwrap_or_default()))
        .collect();

    let mut prev = input;
    for (path, query) in data {
        prev = create_transform(path, prev, query)
    }

    task::spawn(async move {
        for media in prev.into_iter() {
            tx.send(media).expect("reciever should not drop");
        }
    });

    Ok(rx)
}
