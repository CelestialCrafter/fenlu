pub mod fennel;
pub mod filters;
pub mod sources;
pub mod transforms;

use std::{collections::HashMap, fs::File, io::ErrorKind, sync::mpsc::{channel, Receiver}};

use eyre::{Error, Result};
use crate::metadata::Metadata;
use filters::apply_filters;
use sources::{load_sources, create_sources};
use transforms::apply_transforms;
use sqlx::{Connection, SqliteConnection};
use tokio::task::{self};


fn create_db_file() -> Result<()> {
    match File::create_new("fenlu.db") {
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

#[derive(Default)]
pub struct PipelineOpts {
    pub save: bool,
    pub load: bool,
    pub queries: HashMap<String, String>
}

pub async fn run_pipeline(opts: PipelineOpts) -> Result<Receiver<Metadata>> {
    let conn = &mut if opts.save || opts.load {
        let mut conn = SqliteConnection::connect("fenlu.db").await?;
        create_db_file()?;
        create_media_table(&mut conn).await?;

        Some(conn)
    } else {
        None
    };

    let source = if opts.load {
        load_sources(conn.as_mut().unwrap()).await?
    } else {
        create_sources().await?
    };
    let transformed = apply_transforms(source).await?;
    let filtered = apply_filters(transformed, opts.queries).await?;

    let (tx, rx) = channel();

    let handle = task::spawn(async move {
        let conn = &mut if opts.save || opts.load {
            Some(SqliteConnection::connect("fenlu.db").await?)
        } else {
            None
        };

        for media in filtered.into_iter() {
            if opts.save {
                sqlx::query("INSERT OR IGNORE INTO media (uri, metadata) VALUES ($1, $2)")
                    .bind(media.uri.to_string())
                    .bind(serde_json::to_string(&media)?)
                    .execute(conn.as_mut().unwrap())
                    .await?;
            }

            tx.send(media)?;
        }

        Ok::<_, Error>(())
    });

    task::spawn(async {
        handle
            .await
            .expect("handle should not error")
            .expect("transform should not error");
    });

    Ok(rx)
}
