pub mod fennel;
pub mod sources;

use eyre::Result;
use sources::create_merged_source;

#[tokio::main]
async fn main() -> Result<()> {
    let mut source = create_merged_source();
    while let Some(i) = source.recv().await {
        println!("got = {}", i);
    }

    Ok(())
}
