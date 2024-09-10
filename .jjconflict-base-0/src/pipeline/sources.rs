use std::{path::PathBuf, sync::mpsc::{channel, Receiver, Sender}};

use eyre::Result;
use futures::future::try_join_all;
use glob::glob;
use mlua::{Lua, LuaSerdeExt, Value};
use sqlx::{FromRow, SqliteConnection};
use tokio::task;

use crate::metadata::Metadata;

use super::fennel::compile_fennel;

fn create_source(path: PathBuf, tx: Sender<Metadata>) -> Result<()> {
    let (compiled, config) = compile_fennel(path.clone());
    let name = path.file_name().unwrap().to_os_string().into_string().expect("path should be utf-8");

    let lua = unsafe { Lua::unsafe_new() };
    let globals = lua.globals();

    globals.set(
        "add_uri",
        lua.create_function(move |lua, media: Value| {
            let tx = tx.clone();
            let mut media: Metadata = lua.from_value(media)?;
            media.source = name.clone();

            if !media.uri.is_uri() {
                eprintln!("source error: uri {} is invalid", media.uri);
                return Ok(());
            }

            tx.send(media).expect("reciever should not drop");

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

    for row in sqlx::query_as::<_, MetadataRowString>("SELECT metadata FROM media ORDER BY uri DESC").fetch_all(conn).await?.into_iter() {
        let media = serde_json::from_str(row.metadata.as_str()).expect("metadata column should decode to Metadata");
        tx.send(media).expect("reciever should not drop");
    }

    Ok(rx)
}

pub async fn create_sources() -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();
    let mut handles = vec![];

    for path in glob("scripts/*-source.fnl").expect("path read should be valid").map(|path| path.expect("path read should succeed")) {
        let tx = tx.clone();
        handles.push(task::spawn(async move {
            create_source(path, tx)
        }));
    }

    task::spawn(async {
        try_join_all(handles).await.expect("source should succeed");
    });

    Ok(rx)
}
