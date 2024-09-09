use std::{collections::HashMap, sync::mpsc::{channel, Receiver}};

use eyre::{eyre, Error, Result};
use tokio::task;
use toml::{Table, Value};
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
    input: Receiver<Metadata>,
    queries: HashMap<String, String>
) -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();

    let handle = task::spawn(async move {
        let filters: Vec<Filter> = glob("scripts/*-filter.fnl")
            .expect("glob should be valid")
            .map(|path| path.expect("glob should not error"))
            .map(|path| {
                let (compiled, mut config) = compile_fennel(path.clone());
                let name = path.file_name().unwrap().to_os_string().into_string().map_err(|_| eyre!("path is not utf-8"))?;

                if let Some(query) = queries.get(name.as_str()) {
                    let mut table: Table = toml::from_str(config.as_str())?;
                    table.entry("query").or_insert(Value::String(query.to_string()));
                    config = toml::to_string(&table)?;
                }

                Filter::new(compiled, config)
            })
            .collect::<Result<_, Error>>()?;

        for media in input {
            let mut include = true;
            for filter in &filters {
                if !filter.apply(&media)? {
                    include = false;
                    break;
                }
            } 

            if include {
                tx.send(media)?;
            }
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
