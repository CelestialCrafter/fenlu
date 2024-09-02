use std::path::PathBuf;
use eyre::Result;
use mlua::{Function, Lua, LuaSerdeExt, RegistryKey, Value};
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

    pub fn apply(&self, metadata: &Metadata) -> Result<Metadata> {
        let transform_fn: Function = self.lua.registry_value(&self.key)?;
        let value = transform_fn.call::<_, Value>(self.lua.to_value(metadata)?)?;

        Ok(self.lua.from_value(value)?)
    }
}

pub fn apply_transforms<'a>(
    paths: Vec<PathBuf>,
    input: impl Iterator<Item = Metadata> + 'a
) -> Result<impl Iterator<Item = Metadata> + 'a> {
    let mut transforms: Vec<(PathBuf, Result<Transform>)> = paths.into_iter()
        .map(|path| {
            let (compiled, config) = compile_fennel(path.clone()).expect("fennel compilation should not fail");
            let transform = Transform::new(&compiled, &config);
            (path, transform)
        })
        .collect();

    transforms.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let transforms: Vec<Transform> = transforms
        .into_iter()
        .map(|(_, transform)| transform)
        .collect::<Result<Vec<Transform>>>()?;
    
    let output = transforms.into_iter().fold(
        Box::new(input) as Box<dyn Iterator<Item = Metadata> + 'a>,
        |acc, transform| {
            Box::new(acc.map(move |metadata| {
                transform.apply(&metadata).expect("transform should succeed")
            }))
        },
    );

    Ok(output)
}
