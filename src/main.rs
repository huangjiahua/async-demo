#[macro_use]
extern crate log;
extern crate env_logger;

use async_demo::executors::legacy::*;
use failure::Error;
use std::net::SocketAddr;

async fn echo_server() {
    let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
    let mut listener = TcpListener::bind(&addr).expect("cannot bind to address");
    info!("Listening on {:?}", addr);
    while let Ok((stream, addr)) = listener.accept().await {
        info!("connection from {}", addr);
        spawn(echo_handler(stream)).unwrap();
    }
}

async fn echo_handler(mut stream: TcpStream) -> Result<(), Error> {
    while let Ok(content) = stream.read().await {
        if content.is_empty() {
            break;
        }
        stream.write(content).await?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    env_logger::init();
    block_on(echo_server())
}
