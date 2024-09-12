use std::sync::mpsc::{channel, Receiver};

use eyre::{Error, Result};
use tokio::task;
use crate::{metadata::Metadata, utils};
use glob::glob;
use mlua::{Function, Lua, LuaSerdeExt, RegistryKey};

use super::{fennel::compile_fennel, inject_query, Queries};

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
    queries: Queries
) -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();

    let handle = task::spawn(async move {
        let filters: Vec<Filter> = glob("scripts/*-filter.fnl")
            .expect("glob should be valid")
            .map(|path| path.expect("path read should succeed"))
            .filter(|path| utils::is_script_whitelisted(path))
            .map(|path| {
                let query = queries.get(&utils::path_to_name(&path)).cloned().unwrap_or_default();
                let (compiled, mut config) = compile_fennel(path.clone());
                config = inject_query(config, query);

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
            .expect("handle should succeed")
            .expect("transform should succeed");
        });

    Ok(rx)
}
