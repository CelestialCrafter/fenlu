pub mod metadata;
pub mod scripts;
pub mod args;

use std::{fs::File, io::ErrorKind, sync::mpsc::channel};

use args::{parse_args, SourceMode};
use eyre::Result;
use scripts::{filters::apply_filters, sources::{load_sources, create_sources}, transforms::apply_transforms};
use sqlx::{Connection, SqliteConnection};
use tokio::task;
use tracing_subscriber::Registry;


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

#[tokio::main]
async fn main() {
    let args = parse_args().expect("parsing args should succeed");
    let subscriber = Registry::default();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    if let SourceMode::Save = args.source_mode {
        create_db_file().expect("db file creation should succeed");
    }

    let conn = &mut match args.source_mode {
        SourceMode::Calculate => None,
        _ => Some(SqliteConnection::connect("fenlu.db").await.expect("connecting to db should succeed")),
    };

    if let Some(ref mut conn) = conn {
        create_media_table(conn).await.expect("creating media table should succeed");
    }

    let source = match args.source_mode {
        SourceMode::Load => load_sources(conn.as_mut().unwrap(), args.sources).await.expect("loading sources from db should succeed"),
        _ => create_sources(args.sources).await.expect("creating sources should succeed")
    };
    let transformed = apply_transforms(args.transforms, source).expect("applying transforms should succeed");

    let (tx, rx) = channel();

    task::spawn(async {
        let filtered = apply_filters(args.filters, rx.into_iter()).expect("applying filters should succeed");
        for media in filtered {
            let json_string = serde_json::to_string(&media).expect("serializing media to json should succeed");
            println!(
                "{}",
                json_string
            )
        }
    });

    for media in transformed {
        tx.send(media.clone()).expect("reciever should not be dropped");
        if let Some(ref mut conn) = conn {
            if let SourceMode::Save = args.source_mode {
                sqlx::query("INSERT OR IGNORE INTO media (uri, metadata) VALUES ($1, $2)")
                    .bind(media.uri.to_string())
                    .bind(serde_json::to_string(&media).expect("serializing media to json should succeed"))
                    .execute(conn)
                    .await
                    .expect("inserting media to db should succeed");
            }
        }

    }
}
