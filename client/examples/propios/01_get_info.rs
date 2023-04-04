//! cargo run --example 01_get_info --release

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://chrysalis-nodes.iota.org") ?
        .finish()?;

    let info = iota.get_info().await.unwrap();
    println!("Node Info: {info:?}");

    Ok(())

}
