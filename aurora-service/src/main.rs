use anyhow::Result;
use aurora_config::dao_config::Settings;

use sea_orm::Database as SeaDatabase;
use std::net::SocketAddr;

pub mod service;
pub use service::*;

#[tokio::main]
async fn main() -> Result<()> {
    //let _addr: SocketAddr = "0.0.0.0:50051".parse()?;

    // let database_url = env::var("postgres://root:root@tx:16432/Aurorascheduler").expect("DATABASE_URL must be set");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        // .compact()
        // .pretty()
        .init();

    // establish database connection
    let settings = Settings::new()?;
    let url = settings.database.url;
    let host = settings.server.host;
    let port = settings.server.port;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let connection: sea_orm::DatabaseConnection = SeaDatabase::connect(url).await?;

    tracing::info!("aurora-service: connection: {:?}", connection);
    let router = build_router(connection).await;

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
