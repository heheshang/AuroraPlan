use anyhow::{Context, Result};
use lib_conifg::dao_config::Settings;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr};
use tracing::info;
pub mod service;
use lib_common::logger::setup_logger;
use service::*;
#[tokio::main]
async fn main() -> Result<()> {
    //let _addr: SocketAddr = "0.0.0.0:50051".parse()?;
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    setup_logger()?;
    // establish database connection
    let settings = Settings::new()?;
    let url = settings.database.url;
    let host = settings.server.host;
    let port = settings.server.port;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let pool = get_connection_pool(&url).await?;
    info!("database connected");
    let router = build_router(pool).await;

    match listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => {
            listener.set_nonblocking(true)?;
            let listener = tokio_stream::wrappers::TcpListenerStream::new(tokio::net::TcpListener::from_std(listener)?);
            router.serve_with_incoming(listener).await?;
        }
        None => {
            router.serve(addr).await?;
        }
    }

    Ok(())
}
pub async fn get_connection_pool(database_url: &str) -> Result<PgPool> {
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .context("failed to connect to DATABASE_URL")
}
