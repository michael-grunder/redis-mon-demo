use futures::prelude::*;
use redis_async::{
    client::{self, connect::RespConnection},
    resp_array,
};
use std::error::Error;

async fn get_connection(port: u16) -> Result<RespConnection, std::io::Error> {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    client::connect(&addr).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut c1 = get_connection(6379).await?;
    c1.send(resp_array!["MONITOR"]).await?;

    let mut c2 = get_connection(6379).await?;
    c2.send(resp_array!["MONITOR"]).await?;

    // Skip the "OK" response for each "MONITOR" command
    let mut c1 = c1.skip(1);
    let mut c2 = c2.skip(1);

    // This works to monitor the connection `c1` but how could I monitor both
    // `c1` and `c2` as data became available on either stream?
    while let Some(reply) = c1.next().await {
        println!("Reply: {:?}", reply);
    }

    Ok(())
}
