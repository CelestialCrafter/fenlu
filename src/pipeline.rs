use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::ErrorKind,
    path::PathBuf,
    sync::Arc,
};

use crate::{
    protocol::{media, messages::Request},
    script::{self, Script},
    utils,
};
use eyre::{Report, Result};
use sqlx::SqliteConnection;
use tokio::{
    join,
    sync::mpsc,
    task::{self, JoinSet},
};
use tracing::{debug, instrument, Instrument};

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

#[derive(Default)]
pub struct Pipeline {
    pub scripts: HashMap<String, Arc<script::Script>>,
}

impl Pipeline {
    pub async fn populate(&mut self) -> Result<()> {
        let mut set = JoinSet::new();

        let servers = all_scripts().into_iter().map(|path| async move {
            (
                utils::path_to_name(&path),
                script::spawn_server(&path).await,
            )
        });

        for future in servers {
            set.spawn(future);
        }

        while let Some(result) = set.join_next().await {
            let (name, script) = result?;
            self.scripts.insert(name, script?);
        }

        Ok(())
    }

    #[instrument(skip(self, output, buffer_size), name = "pipeline")]
    pub async fn start(
        &self,
        buffer_size: usize,
        output: mpsc::Sender<media::Media>,
    ) -> Result<()> {
        // setup
        let mut source_scripts = vec![];
        let mut transform_scripts = vec![];
        let mut filter_scripts = vec![];

        for script in self.scripts.values() {
            if script.capabilities.media.source {
                source_scripts.push(script.clone());
            } else if script.capabilities.media.transform {
                transform_scripts.push(script.clone());
            } else if script.capabilities.media.filter {
                filter_scripts.push(script.clone());
            }
        }

        let (tx_s, mut rx_s) = mpsc::channel(buffer_size);
        let (tx_t, mut rx_t) = mpsc::channel(buffer_size);

        // sources
        let sources = task::spawn(
            async move {
                for source in source_scripts {
                    let mut state = 0;
                    loop {
                        let response: media::GenerateResponse = serde_json::from_value(
                            source
                                .request(Request {
                                    id: utils::generate_id(),
                                    method: media::GENERATE_METHOD.to_string(),
                                    params: serde_json::to_value(media::GenerateRequest {
                                        batch_size: buffer_size.clone(),
                                        state,
                                    })?,
                                })
                                .await
                                .result()?,
                        )?;

                        for mut media in response.media {
                            append_history(source.path.clone(), &mut media);
                            tx_s.send(media).await?;
                        }

                        state += 1;

                        if response.finished {
                            break;
                        }
                    }
                }

                debug!("sources finished");

                Ok::<_, Report>(())
            }
            .in_current_span(),
        );

        // transforms
        let transforms = task::spawn(
            async move {
                loop {
                    let mut buffer = Vec::with_capacity(buffer_size);
                    if rx_s.recv_many(&mut buffer, buffer_size).await == 0 {
                        break;
                    }

                    for transform in transform_scripts.clone() {
                        buffer = serde_json::from_value(
                            transform
                                .request(Request {
                                    id: utils::generate_id(),
                                    method: media::TRANSFORM_METHOD.to_string(),
                                    params: serde_json::to_value(buffer)?,
                                })
                                .await
                                .result()?,
                        )?;

                        for i in 0..buffer.len() {
                            append_history(transform.path.clone(), &mut buffer[i]);
                        }
                    }

                    for media in buffer {
                        tx_t.send(media).await?;
                    }
                }

                debug!("transforms finished");

                Ok::<_, Report>(())
            }
            .in_current_span(),
        );

        // filters
        let filters = task::spawn(
            async move {
                loop {
                    let mut buffer = Vec::with_capacity(buffer_size);
                    if rx_t.recv_many(&mut buffer, buffer_size).await == 0 {
                        break;
                    }

                    for filter in filter_scripts.clone() {
                        let response: media::FilterResponse = serde_json::from_value(
                            filter
                                .request(Request {
                                    id: utils::generate_id(),
                                    method: media::FILTER_METHOD.to_string(),
                                    params: serde_json::to_value(buffer.clone())?,
                                })
                                .await
                                .result()?,
                        )?;

                        buffer = buffer
                            .into_iter()
                            .enumerate()
                            .filter(|(i, _)| response[*i])
                            .map(|(_, media)| media)
                            .collect();

                        for i in 0..buffer.len() {
                            append_history(filter.path.clone(), &mut buffer[i]);
                        }
                    }

                    for media in buffer {
                        output.send(media).await?;
                    }
                }

                debug!("filters finished");

                Ok::<_, Report>(())
            }
            .in_current_span(),
        );

        let joined = join!(sources, transforms, filters);
        for result in vec![joined.0, joined.1, joined.2] {
            result??;
        }

        Ok(())
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
