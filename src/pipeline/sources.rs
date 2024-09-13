use std::{io::{BufRead, BufReader}, path::PathBuf, process::{Command, Stdio}, sync::mpsc::{channel, Receiver, Sender}};

use eyre::Result;
use futures::future::try_join_all;
use glob::{glob, GlobError};
use sqlx::{FromRow, SqliteConnection};
use tokio::task;

use crate::{metadata::Metadata, utils};

use super::Queries;

fn create_source(path: PathBuf, tx: Sender<Metadata>, query: String) -> Result<()> {
    let name = utils::path_to_name(&path);
    let path = path.canonicalize().expect("path should be canonicalizable");

    let mut dir = path.clone();
    dir.pop();

    let mut child = Command::new(path)
        .arg(query)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("could not start command");
    let stdout = child.stdout.take().expect("could not take stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.expect("should be able to read line");
        let mut media: Metadata = serde_json::from_str(line.as_str()).expect("line should decode to Metadata");
        media.history.push(name.clone());
        tx.send(media).expect("reciever should not drop");
    }

    Ok(())
}

#[derive(FromRow)]
struct MetadataRowString {
    metadata: String
}

pub fn scripts() -> impl Iterator<Item = std::result::Result<PathBuf, GlobError>> {
    glob("scripts/source-*").expect("glob should be valid")
}

pub async fn load_sources(conn: &mut SqliteConnection) -> Result<Receiver<Metadata>> {
    // @TODO load only sources provided by paths arg
    //let sources = paths.into_iter().map(|path| utils::path_to_name(path));
    let (tx, rx) = channel();

    for row in sqlx::query_as::<_, MetadataRowString>("SELECT metadata FROM media ORDER BY uri DESC").fetch_all(conn).await?.into_iter() {
        let media = serde_json::from_str(row.metadata.as_str()).expect("metadata column should decode to Metadata");
        tx.send(media).expect("reciever should not drop");
    }

    Ok(rx)
}

pub async fn create_sources(queries: Queries) -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();
    let mut handles = vec![];

    for path in scripts()
            .map(|path| path.expect("could not read path")) 
            .filter(|path| utils::is_script_whitelisted(path))
        {
            let tx = tx.clone();
            let query = queries.get(&utils::path_to_name(&path)).cloned().unwrap_or_default();

            handles.push(task::spawn(async move {
                create_source(path, tx, query)
            }));
        }

    task::spawn(async {
        try_join_all(handles).await.expect("source should succeed");
    });

    Ok(rx)
}
