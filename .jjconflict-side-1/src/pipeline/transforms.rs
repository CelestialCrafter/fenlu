use std::{path::PathBuf, sync::mpsc::{channel, Receiver}};
use eyre::{Error, Result};
use glob::glob;
use mlua::{Function, Lua, LuaSerdeExt, RegistryKey, Value};
use tokio::task;
use crate::metadata::Metadata;
use super::fennel::compile_fennel;

pub struct Transform {
    lua: Lua,
    key: RegistryKey,
}

impl Transform {
    pub fn new(compiled: &str, config: &str) -> Result<Self> {
        let lua = unsafe { Lua::unsafe_new() };
        let transform_fn = lua.load(compiled).call::<String, Function>(config.to_string())?;
        let key = lua.create_registry_value(transform_fn)?;

        Ok(Transform { lua, key })
    }

    pub fn apply(&self, media: &Metadata) -> Result<Metadata> {
        let transform_fn: Function = self.lua.registry_value(&self.key)?;
        let value = transform_fn.call::<_, Value>(self.lua.to_value(media)?)?;

        Ok(self.lua.from_value(value)?)
    }
}

pub async fn apply_transforms<'a>(
    input: Receiver<Metadata>
) -> Result<Receiver<Metadata>> {
    let (tx, rx) = channel();

    let handle = task::spawn(async move {
        let mut transforms: Vec<(PathBuf, Result<Transform>)> = glob("scripts/*-transform.fnl")
            .expect("glob should be valid")                                                                            
            .map(|path| path.expect("path read should succeed"))
            .map(|path| {
                let (compiled, config) = compile_fennel(path.clone());
                let transform = Transform::new(&compiled, &config);
                (path, transform)
            })
        .collect();

        transforms.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let transforms: Vec<Transform> = transforms
            .into_iter()
            .map(|(_, transform)| transform)
            .collect::<Result<Vec<Transform>>>()?;

        for media in input {
            let mut output = media;
            for transform in &transforms {
                output = transform.apply(&output)?;
            } 

            tx.send(output)?;
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
