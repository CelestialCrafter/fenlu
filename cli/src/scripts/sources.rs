use std::{path::PathBuf, sync::mpsc::{channel, Sender}};

use eyre::Result;
use mlua::{ExternalResult, Lua, LuaSerdeExt, Value};
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
                Err("uri is invalid").into_lua_err()?;
            }

            tx.send(metadata).expect("reciever should not drop");

            Ok(())
        })?,
    )?;

    lua.load(&compiled).call(config)?;

    Ok(())
}

pub async fn apply_sources(paths: Vec<PathBuf>) -> Result<impl Iterator<Item = Metadata>> {
    let (tx, rx) = channel();
    let mut handles = vec![];

    for path in paths {
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

    Ok(rx.into_iter())
}
