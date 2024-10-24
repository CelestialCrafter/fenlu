use std::{
    collections::HashMap,
    fs::read_dir,
    path::PathBuf,
    sync::{Arc, LazyLock}, time::Duration,
};

use crate::{
    protocol::{capabilities, media, messages::Request},
    script::{self},
    utils,
};
use eyre::{Report, Result};
use tokio::{
    join, sync::mpsc, task::{self}, time
};
use tracing::{info, instrument, Instrument};

pub const DB_PATH: &str = "fenlu.db";

pub static GLOBAL_PIPELINE: LazyLock<Pipeline> = LazyLock::new(|| {
    let mut pipeline = Pipeline::default();
    pipeline.populate().expect("could not populate pipeline");
    pipeline
});

pub fn append_history(path: &PathBuf, data: serde_json::Value, media: &mut media::Media) {
    let name = utils::path_to_name(path);
    media.history.insert(name, data);
}

#[derive(Default)]
pub struct Pipeline {
    pub scripts: HashMap<String, Arc<script::Script>>,
}

impl Pipeline {
    pub fn populate(&mut self) -> Result<()> {
        for path in all_scripts() {
            let name = utils::path_to_name(&path);
            let script = script::spawn_server(&path);

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
            match script.capabilities.media.0 {
                capabilities::Type::Source => source_scripts.push(script.clone()),
                capabilities::Type::Transform => transform_scripts.push(script.clone()),
                capabilities::Type::Filter => filter_scripts.push(script.clone()),
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
                        let mut response: media::GenerateResponse = serde_json::from_value(
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

                        response.extra.resize(response.media.len(), serde_json::Value::Null);
                        for (mut media, extra) in response.media.into_iter().zip(response.extra) {
                            append_history(&source.path, extra, &mut media);
                            tx_s.send(media).await?;
                        }

                        state += 1;

                        if response.finished {
                            break;
                        }

                        if let Some(ms) = source.capabilities.media.1 {
                            time::sleep(Duration::from_millis(ms)).await;
                        }
                    }
                }

                info!("sources finished");

                Ok::<_, Report>(())
            }
        .in_current_span(),
        );

        // transforms
        let transforms = task::spawn(
            async move {
                loop {
                    let mut buffer: Vec<media::Media> = Vec::with_capacity(buffer_size);
                    if rx_s.recv_many(&mut buffer, buffer_size).await == 0 {
                        break;
                    }

                    for transform in transform_scripts.clone() {
                        let expected_len = buffer.len();

                        let mut response: media::TransformResponse = serde_json::from_value(
                            transform
                            .request(Request {
                                id: utils::generate_id(),
                                method: media::TRANSFORM_METHOD.to_string(),
                                params: serde_json::to_value(buffer)?,
                            })
                            .await
                            .result()?,
                        )?;
                        assert!(response.media.len() == expected_len, "media amount was not equal to buffer length");

                        buffer = response.media;

                        response.extra.resize(buffer.len(), serde_json::Value::Null);
                        for (i, extra) in response.extra.into_iter().enumerate() {
                            append_history(&transform.path, extra, &mut buffer[i]);
                        }

                        if let Some(ms) = transform.capabilities.media.1 {
                            time::sleep(Duration::from_millis(ms)).await;
                        }
                    }

                    for media in buffer {
                        tx_t.send(media).await?;
                    }
                }

                info!("transforms finished");

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
                        let mut response: media::FilterResponse = serde_json::from_value(
                            filter
                            .request(Request {
                                id: utils::generate_id(),
                                method: media::FILTER_METHOD.to_string(),
                                params: serde_json::to_value(buffer.clone())?,
                            })
                            .await
                            .result()?,
                        )?;

                        assert!(response.included.len() == buffer.len(), "included items was not equal to buffer length");

                        buffer = buffer
                            .into_iter()
                            .enumerate()
                            .filter(|(i, _)| response.included[*i])
                            .map(|(_, media)| media)
                            .collect();

                        response.extra.resize(buffer.len(), serde_json::Value::Null);
                        for (i, extra) in response.extra.into_iter().enumerate() {
                            append_history(&filter.path, extra, &mut buffer[i]);
                        }

                        if let Some(ms) = filter.capabilities.media.1 {
                            time::sleep(Duration::from_millis(ms)).await;
                        }
                    }

                    for media in buffer {
                        output.send(media.clone()).await?;
                    }
                }

                info!("filters finished");

                Ok::<_, Report>(())
            }
        .in_current_span(),
        );

        let joined = join!(sources, transforms, filters);
        for result in vec![joined.0, joined.1, joined.2] {
            // @TODO just return all 3 errors incase multiple tasks fail
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
