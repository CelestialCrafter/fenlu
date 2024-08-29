use std::{path::PathBuf, sync::Arc};

use eyre::Result;
use fluent_uri::UriRef;
use glob::glob;
use mlua::{ExternalResult, Lua, MultiValue};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task,
};

use super::fennel::compile_fennel;

async fn create_source(path: PathBuf, tx: Arc<Sender<UriRef<String>>>) -> Result<()> {
    let (compiled, config) = compile_fennel(path).expect("fennel compilation should not fail");

    unsafe {
        let lua = Lua::unsafe_new();
        let globals = lua.globals();

        globals.set(
            "add_uri",
            lua.create_function(move |_, uri_string: MultiValue| {
                let tx = tx.clone();

                let uri = UriRef::parse(uri_string).into_lua_err()?;
                if !uri.is_uri() {
                    Err("uri is invalid").into_lua_err()?;
                }

                task::spawn(async move {
                    tx.send(uri).await.expect("reciever should not drop");
                });

                Ok(())
            })?,
        )?;

        lua.load(&compiled).call(config)?;
    }

    Ok(())
}

pub fn create_merged_source() -> Receiver<UriRef<String>> {
    // @TODO find good buffer size
    let (tx, rx) = mpsc::channel(1000);
    let tx = Arc::new(tx);
    let mut handles = vec![];

    for path in glob("scripts/*-source.fnl").expect("glob should be valid") {
        handles.push(task::spawn(create_source(
            path.expect("glob should not error"),
            tx.clone()
        )));
    }

    task::spawn(async {
        for handle in handles {
            handle
                .await
                .expect("handle should not error")
                .expect("source should not error");
        }
    });

    rx
}
