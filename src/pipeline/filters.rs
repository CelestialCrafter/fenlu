use std::sync::mpsc::{channel, Receiver};

use eyre::{Error, Result};
use tokio::task;
use crate::metadata::Metadata;
use glob::glob;
use mlua::{Function, Lua, LuaSerdeExt, RegistryKey};

use super::fennel::compile_fennel;

pub struct Filter {
    lua: Lua,
    key: RegistryKey,
}

impl Filter {
    pub fn new(compiled: String, config: String) -> Result<Self> {
        let lua = unsafe { Lua::unsafe_new() };
        let filter_fn = lua.load(compiled).call::<String, Function>(config)?;
        let key = lua.create_registry_value(filter_fn)?;

        Ok(Filter { lua, key })
    }

    pub fn apply(&self, media: &Metadata) -> Result<bool> {
        let filter_fn: Function = self.lua.registry_value(&self.key)?;
        Ok(filter_fn.call::<_, bool>(self.lua.to_value(media)?)?)
    }
}

pub async fn apply_filters(
    input: Receiver<Metadata>
) -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();

    let handle = task::spawn(async move {
        let filters: Vec<Filter> = glob("scripts/*-filter.fnl")
            .expect("glob should be valid")
            .map(|path| path.expect("glob should not error"))
            .map(|path| {
                let (compiled, config) = compile_fennel(path.clone()).expect("fennel compilation should not fail");
                Filter::new(compiled, config)
            })
        .collect::<Result<Vec<Filter>>>()?;

        for media in input {
            for filter in &filters {
                if !filter.apply(&media)? {
                    continue;
                }
            } 

            tx.send(media)?;
        }

        Ok::<_, Error>(())
    });

    task::spawn(async {
        handle
            .await
            .expect("handle should not error")
            .expect("transform should not error");
    });

    Ok(rx)
}
