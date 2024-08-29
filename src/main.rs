pub mod scripts;

use eyre::Result;
use scripts::sources::create_merged_source;

#[tokio::main]
async fn main() -> Result<()> {
    let mut source = create_merged_source();
    while let Some(i) = source.recv().await {
        println!("got = {}", i);
    }

    Ok(())
}
