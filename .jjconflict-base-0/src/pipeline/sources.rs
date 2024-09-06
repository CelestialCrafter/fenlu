use std::{path::PathBuf, sync::mpsc::{channel, Receiver, Sender}};

use eyre::Result;
use glob::glob;
use mlua::{Lua, LuaSerdeExt, Value};
use sqlx::{FromRow, SqliteConnection};
use tokio::task;

use crate::metadata::Metadata;

use super::fennel::compile_fennel;

fn create_source(path: PathBuf, tx: Sender<Metadata>) -> Result<()> {
    let (compiled, config) = compile_fennel(path.clone()).expect("fennel compilation should not fail");
    let name = path.file_name().unwrap().to_os_string().into_string().expect("path should be utf-8");

    let lua = unsafe { Lua::unsafe_new() };
    let globals = lua.globals();

    globals.set(
        "add_uri",
        lua.create_function(move |lua, metadata: Value| {
            let tx = tx.clone();
            let mut metadata: Metadata = lua.from_value(metadata)?;
            metadata.source = name.clone();

            if !metadata.uri.is_uri() {
                eprintln!("source error: uri {} is invalid", metadata.uri);
                return Ok(());
            }

            tx.send(metadata).expect("reciever should not drop");

            Ok(())
        })?,
    )?;

    lua.load(&compiled).call(config)?;

    Ok(())
}

#[derive(FromRow)]
struct MetadataRowString {
    metadata: String
}

pub async fn load_sources(conn: &mut SqliteConnection) -> Result<Receiver<Metadata>> {
    // @TODO load only sources provided by paths arg
    //let sources = paths.into_iter().map(|path| path.file_name().unwrap().to_os_string().into_string().expect("path should be utf-8"));
    let (tx, rx) = channel();

    for row in sqlx::query_as::<_, MetadataRowString>("SELECT metadata FROM media").fetch_all(conn).await?.into_iter() {
        let metadata = serde_json::from_str(row.metadata.as_str()).expect("metadata column should decode to Metadata");
        tx.send(metadata).expect("reciever should not be dropped");
    }

    Ok(rx)
}

pub async fn create_sources() -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();
    let mut handles = vec![];

    for path in glob("scripts/*-source.fnl").expect("glob should be valid").map(|p| p.expect("fennel compilation should not fail")) {
        let tx = tx.clone();
        handles.push(task::spawn(async move {
            create_source(path, tx)
        }));
    }

    task::spawn(async {
        for handle in handles {
            handle
                .await
                .expect("handle should not error")
                .expect("source should not error");
            }
    });

    Ok(rx)
}
