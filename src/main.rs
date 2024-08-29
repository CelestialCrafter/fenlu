pub mod metadata;
pub mod scripts;

use std::{fs::File, io::ErrorKind};

use eyre::Result;
use scripts::sources::create_merged_source;
use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<()> {
    match File::create_new("fenlu.db") {
        Ok(_) => Ok(()),
        Err(error) => {
            match error.kind() {
                ErrorKind::AlreadyExists => Ok(()),
                _ => Err(error)
            }
        }
    }?;

    let mut conn = SqliteConnection::connect("fenlu.db").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media (
            uri TEXT PRIMARY KEY,
            metadata TEXT NOT NULL
    )",
    )
    .execute(&mut conn)
    .await?;

    let mut source = create_merged_source();

    while let Some(media) = source.recv().await {
        sqlx::query("INSERT INTO media (uri, metadata) VALUES ($1, $2)")
            .bind(media.uri.to_string())
            .bind(serde_json::to_string(&media)?)
            .execute(&mut conn)
            .await?;
        println!("got = {:?}", serde_json::to_string(&media)?);
    }

    Ok(())
}
