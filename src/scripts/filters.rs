use eyre::Result;
use glob::glob;
use mlua::{Function, Lua, LuaSerdeExt, RegistryKey};
use crate::metadata::Metadata;
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

    pub fn apply(&self, metadata: &Metadata) -> Result<bool> {
        let filter_fn: Function = self.lua.registry_value(&self.key)?;
        Ok(filter_fn.call::<_, bool>(self.lua.to_value(metadata)?)?)
    }
}

pub fn apply_filters<'a>(
    input: impl Iterator<Item = Metadata> + 'a
) -> Result<impl Iterator<Item = Metadata> + 'a> {
    let filters: Vec<Filter> = glob("scripts/*-filter.fnl")
        .expect("glob should be valid")
        .map(|path| path.expect("glob should not error"))
        .map(|path| {
            let (compiled, config) = compile_fennel(path.clone()).expect("fennel compilation should not fail");
            Filter::new(compiled, config)
        })
        .collect::<Result<Vec<Filter>>>()?;

    let output = filters.into_iter().fold(
        Box::new(input) as Box<dyn Iterator<Item = Metadata> + 'a>,
        |acc, filter| {
            Box::new(acc.filter(move |metadata| {
                filter.apply(metadata).expect("filter should succeed")
            }))
        },
    );

    Ok(output)
}
