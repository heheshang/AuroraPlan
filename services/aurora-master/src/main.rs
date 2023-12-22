use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use lib_common::logger::setup_logger;
use lib_conifg::master_config::Settings;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;
    let listen_port = settings.master.listen_port;
    let _ = tokio::join!(start(host, port), listen(listen_port));

    info!("master  start success!");
    Ok(())
}
async fn listen(listen_port: u32) -> Result<()> {
    info!("master  start  {}", listen_port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", listen_port)).await?;

    loop {
        let (_socket, _) = listener.accept().await?;

        // process(socket ).await;
    }
}

async fn start(host: String, port: u32) -> Result<()> {
    info!("master axum  start {}:{}", host, port);
    // let backtrace = backtrace::Backtrace::new();
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let router = Router::new().route("/", get(|| async { "Hello, world!" }));
    let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(tcp_listener, router).await?;

    // println!("log init success! {:#?}", backtrace);
    Ok(())
}
