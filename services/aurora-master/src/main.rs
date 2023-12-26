use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use lib_common::logger::setup_logger;
use lib_conifg::master_config::Settings;
use tracing::info;
pub mod rpc;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;
    let _ = tokio::join!(start(host, port), listen());

    info!("master  start success!");
    Ok(())
}
async fn listen() -> Result<()> {
    rpc::MasterRpcClient::start().await?;
    rpc::MasterRpcServer::start().await?;
    Ok(())
}

async fn start(host: String, port: u32) -> Result<()> {
    info!("master axum  start {}:{}", host, port);
    // let backtrace = backtrace::Backtrace::new();
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let router = Router::new().route(
        "/",
        get(|| async {
            info!("hello world");
            "Hello, world!"
        }),
    );
    let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(tcp_listener, router).await?;

    // println!("log init success! {:#?}", backtrace);
    Ok(())
}
