pub mod metadata;
pub mod scripts;

use std::{fs::File, io::ErrorKind, sync::mpsc::channel};

use eyre::Result;
use scripts::{filters::apply_filters, sources::apply_sources, transforms::apply_transforms};
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

#[tokio::main]
async fn main() {
    let subscriber = Registry::default();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    create_db_file().expect("db file creation should succeed");

    let mut conn = SqliteConnection::connect("fenlu.db").await.expect("connecting to db should succeed");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media (
            uri TEXT PRIMARY KEY,
            metadata TEXT NOT NULL
        )",
    )
        .execute(&mut conn)
        .await.expect("creating media table should succeed");

    let source = apply_sources().await.expect("applying sources should succeed");
    let transformed = apply_transforms(source).expect("applying transforms should succeed");

    let (tx, rx) = channel();

    task::spawn(async {
        let filtered = apply_filters(rx.into_iter()).expect("applying filters should succeed");
        for media in filtered {
            println!(
                "got = {:?}",
                serde_json::to_string(&media).expect("serializing media to json should succees")
            )
        }
    });

    for media in transformed {
        tx.send(media.clone()).expect("reciever should not be dropped");
        sqlx::query("INSERT OR IGNORE INTO media (uri, metadata) VALUES ($1, $2)")
            .bind(media.uri.to_string())
            .bind(serde_json::to_string(&media).expect("serializing media to json should succeed"))
            .execute(&mut conn)
            .await
            .expect("inserting media to db should succeed");
    }
}
