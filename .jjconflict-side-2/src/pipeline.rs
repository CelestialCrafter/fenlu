use std::{collections::HashMap, fs::{read_dir, File}, io::ErrorKind, path::PathBuf, sync::Arc};

use eyre::Result;
use crate::{protocol::{media, messages::Request, query}, script, utils};
use sqlx::SqliteConnection;
use tokio::{sync::mpsc, task};

pub const DB_PATH: &str = "fenlu.db";

fn create_db_file() -> Result<()> {
    match File::create_new(DB_PATH) {
        Ok(_) => Ok(()),
        Err(error) => {
            match error.kind() {
                ErrorKind::AlreadyExists => Ok(()),
                _ => Err(error.into())
            }
        }
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

pub struct Pipeline {
    pub scripts: Vec<Arc<script::Script>>,
}

impl Pipeline {
    pub async fn set_queries(&self, queries: Queries) -> Result<()> {
        for script in self.scripts.clone() {
            if !script.capabilities.query.query {
                continue;
            }

            let script_query = match queries.get(&utils::path_to_name(&script.path)) {
                Some(query) => query,
                None => continue
            }.clone();


            script.request(Request {
                id: utils::generate_id(),
                method: query::QUERY_METHOD.to_string(),
                params: serde_json::to_value(query::QueryRequest {
                   query: script_query 
                })?
            });
        }

        Ok(())
    }

    pub async fn run(&self, buffer_size: usize, output: mpsc::Sender<media::Media>) -> Result<()> {
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

        let (tx_s, mut rx_s) = mpsc::channel(buffer_size);
        let (tx_t, mut rx_t) = mpsc::channel(buffer_size);

        let sources = task::spawn(async move {
            for source in source_scripts {
                loop {
                    let response: media::GenerateResponse = serde_json::from_value(source.request(Request {
                        id: utils::generate_id(),
                        method: media::GENERATE_METHOD.to_string(),
                        params: serde_json::to_value(media::GenerateRequest {
                            batch_size: buffer_size.clone() as u32
                        })?
                    }).await?.result()?)?;

                    for media in response.media {
                        tx_s.send(media).await?;
                    }

                    if response.finished {
                        break;
                    }
                }
            }

            Ok::<_, eyre::Report>(())
        });

        // transforms
        let transforms = task::spawn(async move {
            while let Some(mut media) = rx_s.recv().await {
                for transform in transform_scripts.clone() {
                    media = serde_json::from_value(transform.request(Request {
                        id: utils::generate_id(),
                        method: media::TRANSFORM_METHOD.to_string(),
                        params: serde_json::to_value(media)?
                    }).await?.result()?)?;
                }

                tx_t.send(media).await?;
            }
            Ok::<_, eyre::Report>(())
        });

        // filters
        let filters = task::spawn(async move {
            while let Some(media) = rx_t.recv().await {
                let mut included = true;
                let value = serde_json::to_value(media.clone())?;

                for filter in filter_scripts.clone() {
                    let response: media::FilterResponse = serde_json::from_value(filter.request(Request {
                        id: utils::generate_id(),
                        method: media::FILTER_METHOD.to_string(),
                        params: value.clone()
                    }).await?.result()?)?;

                    if !response.included {
                        included = false;
                        break
                    }
                }

                if included {
                    output.send(media).await?;
                }
            }

            Ok::<_, eyre::Report>(())
        });

        sources.await?;
        transforms.await?;
        filters.await?;

        Ok(())
    }
}

pub fn all_scripts() -> Vec<PathBuf> {
    read_dir("scripts/").expect("could not read scripts directory")
        .collect::<Result<Vec<_>, _>>().expect("could not collect dir entries")
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| utils::is_script_whitelisted(path))
        .collect()
}

pub type Queries = HashMap<String, String>;
pub async fn spawn_all_scripts() -> Result<Pipeline> {
    Ok(Pipeline {
        scripts: all_scripts()
            .into_iter()
            .map(|path| script::spawn_server(path))
            .collect::<Result<Vec<Arc<script::Script>>>>()?
    })
}
