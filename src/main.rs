use futures::prelude::*;
use redis_async::client::connect::RespConnection;
use redis_async::{client, resp_array};
use std::error::Error;

async fn get_connection(port: u16) -> Result<RespConnection, std::io::Error> {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    client::connect(&addr).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port1 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "6379".to_string())
        .parse()
        .unwrap();

    let port2 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "6379".to_string())
        .parse()
        .unwrap();

    let mut c1 = get_connection(port1).await?;
    c1.send(resp_array!["MONITOR"]).await?;

    let mut c2 = get_connection(port2).await?;
    c2.send(resp_array!["MONITOR"]).await?;

    // Skip the "OK" response for each "MONITOR" command
    let mut skip1 = c1.skip(1);
    let mut skip2 = c2.skip(1);

    // Simple enough to wait on one stream of replies
    while let Some(reply) = skip1.next().await {
        println!("Reply: {:?}", reply);
    }

    Ok(())
}
