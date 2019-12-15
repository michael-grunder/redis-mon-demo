use futures::{executor::block_on, prelude::*, task};
use redis_async::client::connect::RespConnection;
use redis_async::{client, resp_array};
use std::{error::Error, io::Read};

async fn get_connection(port: u16) -> Result<RespConnection, std::io::Error> {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    client::connect(&addr).await
}

async fn next_reply(
    con: &mut RespConnection,
) -> Option<Result<redis_async::resp::RespValue, redis_async::error::Error>> {
    con.next().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut c1 = get_connection(6379).await?;
    c1.send(resp_array!["MONITOR"]).await?;

    let mut skip = c1.skip(1);
    while let Some(resp) = skip.next().await {
        println!("Response: {:?}", resp);
    }

    Ok(())
}
