pub mod sources;
pub mod transforms;

use std::{collections::HashMap, fs::File, io::ErrorKind, sync::mpsc::{channel, Receiver}};

use eyre::{Error, Result};
use crate::{config::{PipelineMode, CONFIG}, metadata::Metadata};
use transforms::apply_transforms;
use sources::{load_sources, create_sources};
use sqlx::{Connection, SqliteConnection};
use tokio::task::{self};

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

pub type Queries = HashMap<String, String>;
pub async fn run_pipeline(queries: Queries) -> Result<Receiver<Metadata>> {
    // generate: source (create) -> transform
    // load: db -> source (load)
    // generate_save: db -> source (create) -> transform -> save

    let conn = &mut if let PipelineMode::Generate = CONFIG.pipeline_mode {
        None
    } else {
        // make sure load is checked before create_db_file is called
        // load is enabled by default only if the db file exists
        // to allow for generation on first app run
        create_db_file()?;

        let mut conn = SqliteConnection::connect("fenlu.db").await?;
        create_media_table(&mut conn).await?;

        Some(conn)
    };

    let source = if let PipelineMode::Load = CONFIG.pipeline_mode {
        load_sources(conn.as_mut().unwrap()).await?
    } else {
        create_sources(queries.clone()).await?
    };

    let transformed = if let PipelineMode::Load = CONFIG.pipeline_mode {
        source
    } else {
        apply_transforms(source, queries.clone()).await?
    };

    if let PipelineMode::GenerateSave = CONFIG.pipeline_mode {
        let (tx, rx) = channel();

        let handle = task::spawn(async move {
            let mut conn = SqliteConnection::connect("fenlu.db").await?;
            for media in transformed.into_iter() {
                sqlx::query("INSERT OR IGNORE INTO media (uri, metadata) VALUES ($1, $2)")
                    .bind(media.uri.to_string())
                    .bind(serde_json::to_string(&media)?)
                    .execute(&mut conn)
                    .await?;

                tx.send(media)?;
            }

            Ok::<_, Error>(())
        });

        task::spawn(async {
            handle
                .await
                .expect("handle should succeed")
                .expect("could not save to db");
        });

        Ok(rx)
    } else {
        Ok(transformed)
    }
}
