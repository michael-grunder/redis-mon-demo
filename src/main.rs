use futures::{prelude::*, stream::SelectAll};
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
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let mut connections = SelectAll::new();

    for arg in &args {
        let mut con = get_connection(arg.parse().unwrap()).await?;
        con.send(resp_array!["MONITOR"]).await?;
        let con = con.skip(1);
        connections.push(con);
    }

    while let Some(v) = connections.next().await {
        println!("{:?}", v);
    }

    Ok(())
}
