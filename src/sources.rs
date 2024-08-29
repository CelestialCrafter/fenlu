use crate::fennel::compile_fennel;
use eyre::Error;
use fluent_uri::UriRef;
use glob::glob;
use mlua::{ExternalResult, Lua};
use tokio::{sync::mpsc::{self, Receiver}, task};

pub fn create_merged_source() -> Receiver<UriRef<String>> {
    // @TODO find good buffer size
    let (tx, rx) = mpsc::channel(1000);
    let mut handles = vec![];

    for script_path in glob("scripts/*-source.fnl").expect("glob should be valid") {
        let tx = tx.clone();
        let script = async move {
            let lua = Lua::new();
            let globals = lua.globals();

            globals.set(
                "add_uri",
                lua.create_function(move |_, uri_string: String| {
                    let tx = tx.clone();
                    let uri = UriRef::parse(uri_string).into_lua_err()?;
                    if !uri.is_uri() {
                        Err("uri is invalid").into_lua_err()?;
                    }
                    task::spawn(async move {
                        tx.send(uri).await.expect("reciever should not drop");
                    });
                    Ok(())
                })?
            )?;

            let compiled = compile_fennel(script_path?).expect("fennel compilation should not fail");
            lua.load(&compiled).exec()?;
            Ok::<(), Error>(())
        };

        handles.push(task::spawn(script));
    }

    task::spawn(async {
        for handle in handles {
            let _ = handle.await.expect("source should not error");
        }
    });

    rx
}
