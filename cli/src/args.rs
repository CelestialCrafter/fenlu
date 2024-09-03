use std::{path::PathBuf, str::FromStr};

use eyre::{eyre, Error, Result};
use globset::Glob;
use pico_args::Arguments;

#[derive(Debug)]
pub struct Args {
    pub source_mode: SourceMode,
    pub sources: Vec<PathBuf>,
    pub transforms: Vec<PathBuf>,
    pub filters: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum SourceMode {
    Load,
    Save,
    Calculate,
}

impl FromStr for SourceMode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "load" => Ok(Self::Load),
            "save" => Ok(Self::Save),
            "calculate" => Ok(Self::Calculate),
            _ => Err(eyre!("could not match source mode"))
        }
    }
}

impl Default for SourceMode {
    fn default() -> Self {
        Self::Calculate
    }
}

#[derive(Debug)]
enum Destination {
    Source,
    Transform,
    Filter,
}

pub fn parse_args() -> Result<Args> {
    let mut pico_args = Arguments::from_env();
    let mut args = Args {
        source_mode: pico_args.value_from_str("--source-mode").unwrap_or(pico_args.value_from_str("-m").unwrap_or_default()),
        sources: vec![],
        transforms: vec![],
        filters: vec![]
    };

    let mut sources = vec![];
    let mut transforms = vec![];
    let mut filters = vec![];

    let mut result = Ok(None);
    while let Ok(data) = result {
        if let Some((dest, path)) = data {
            match dest {
                Destination::Source => sources.push(path),
                Destination::Transform => transforms.push(path),
                Destination::Filter => filters.push(path),
            };
        }

        result = pico_args.free_from_fn::<Option<(Destination, PathBuf)>, Error>(|path| {
            let source_glob = Glob::new("*-source.fnl")?.compile_matcher();
            let transform_glob = Glob::new("*-transform.fnl")?.compile_matcher();
            let filter_glob = Glob::new("*-filter.fnl")?.compile_matcher();

            let path: PathBuf = path.into();
            Ok(if source_glob.is_match(path.clone()) {
                Some((Destination::Source, path))
            } else if transform_glob.is_match(path.clone()) {
                Some((Destination::Transform, path))
            } else if filter_glob.is_match(path.clone()) {
                Some((Destination::Filter, path))
            } else {
                None
            })
        });
    }

    args.sources = sources;
    args.filters = filters;
    args.transforms = transforms;

    Ok(args)
}
