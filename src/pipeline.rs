use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::ErrorKind,
    path::PathBuf,
    sync::Arc,
};

use crate::{
    protocol::{media, messages::Request, query},
    script, utils,
};
use eyre::{Report, Result};
use sqlx::SqliteConnection;
use tokio::{
    join,
    sync::mpsc::{channel, Receiver},
    task::{self, JoinSet},
};
use tracing::debug_span;

pub const DB_PATH: &str = "fenlu.db";

fn create_db_file() -> Result<()> {
    match File::create_new(DB_PATH) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(error.into()),
        },
    }
}

async fn create_media_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media (
            uri TEXT PRIMARY KEY,
            metadata TEXT NOT NULL
        )",
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub fn append_history(path: PathBuf, media: &mut media::Media) {
    let name = utils::path_to_name(&path);
    media.history.push(name);
}

#[derive(Debug, Default)]
pub struct Pipeline {
    pub scripts: Vec<Arc<script::Script>>,
}

impl Pipeline {
    pub async fn populate(&mut self) -> Result<()> {
        let mut set = JoinSet::new();

        let script_paths = all_scripts();
        let mut scripts = Vec::with_capacity(script_paths.len());

        let servers = script_paths
            .into_iter()
            .map(|path| script::spawn_server(path));

        for future in servers {
            set.spawn(future);
        }

        while let Some(script) = set.join_next().await {
            scripts.push(script??);
        }

        self.scripts = scripts;

        Ok(())
    }

    pub async fn set_queries(&self, queries: &HashMap<String, String>) -> Result<()> {
        for script in self.scripts.clone() {
            if !script.capabilities.query.query {
                continue;
            }

            let script_query = match queries.get(&utils::path_to_name(&script.path)) {
                Some(query) => query,
                None => continue,
            }
            .clone();

            script
                .request(Request {
                    id: utils::generate_id(),
                    method: query::QUERY_METHOD.to_string(),
                    params: serde_json::to_value(query::QueryRequest {
                        query: script_query,
                    })?,
                })
                .await;
        }

        Ok(())
    }

    pub async fn run(&self, buffer_size: usize) -> Receiver<media::Media> {
        // setup
        let mut source_scripts = vec![];
        let mut transform_scripts = vec![];
        let mut filter_scripts = vec![];

        for script in self.scripts.clone() {
            if script.capabilities.media.source {
                source_scripts.push(script);
            } else if script.capabilities.media.transform {
                transform_scripts.push(script);
            } else if script.capabilities.media.filter {
                filter_scripts.push(script);
            }
        }

        let (tx_s, mut rx_s) = channel(buffer_size);
        let (tx_t, mut rx_t) = channel(buffer_size);
        let (output_tx, output_rx) = channel(buffer_size);

        let sources = task::spawn(async move {
            for source in source_scripts {
                loop {
                    let name = utils::path_to_name(&source.path);
                    let _ = debug_span!("requesting source", name = name).enter();

                    let response: media::GenerateResponse = serde_json::from_value(
                        source
                            .request(Request {
                                id: utils::generate_id(),
                                method: media::GENERATE_METHOD.to_string(),
                                params: serde_json::to_value(media::GenerateRequest {
                                    batch_size: buffer_size.clone() as u32,
                                })?,
                            })
                            .await
                            .result()?,
                    )?;

                    for mut media in response.media {
                        append_history(source.path.clone(), &mut media);
                        tx_s.send(media).await?;
                    }

                    if response.finished {
                        break;
                    }
                }
            }

            Ok::<_, Report>(())
        });

        // transforms
        let transforms = task::spawn(async move {
            while let Some(mut media) = rx_s.recv().await {
                for transform in transform_scripts.clone() {
                    let name = utils::path_to_name(&transform.path);
                    let _ = debug_span!("requesting transform", name = name).enter();

                    media = serde_json::from_value(
                        transform
                            .request(Request {
                                id: utils::generate_id(),
                                method: media::TRANSFORM_METHOD.to_string(),
                                params: serde_json::to_value(media)?,
                            })
                            .await
                            .result()?,
                    )?;
                    append_history(transform.path.clone(), &mut media);
                }

                tx_t.send(media).await?;
            }
            Ok::<_, Report>(())
        });

        // filters
        let filters = task::spawn(async move {
            while let Some(mut media) = rx_t.recv().await {
                let mut included = true;
                let value = serde_json::to_value(media.clone())?;

                for filter in filter_scripts.clone() {
                    let name = utils::path_to_name(&filter.path);
                    let _ = debug_span!("requesting filter", name = name).enter();

                    let response: media::FilterResponse = serde_json::from_value(
                        filter
                            .request(Request {
                                id: utils::generate_id(),
                                method: media::FILTER_METHOD.to_string(),
                                params: value.clone(),
                            })
                            .await
                            .result()?,
                    )?;

                    if !response.included {
                        included = false;
                        break;
                    }

                    append_history(filter.path.clone(), &mut media);
                }

                if included {
                    output_tx.send(media).await?;
                }
            }

            Ok::<_, Report>(())
        });

        task::spawn(async {
            let joined = join!(sources, transforms, filters);
            for result in vec![joined.0, joined.1, joined.2] {
                result.unwrap().unwrap();
            }
        });

        output_rx
    }
}

pub fn all_scripts() -> Vec<PathBuf> {
    read_dir("scripts/")
        .expect("could not read scripts directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("could not collect dir entries")
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| utils::is_script_whitelisted(path))
        .collect()
}
