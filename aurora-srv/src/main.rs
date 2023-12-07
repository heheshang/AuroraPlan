use anyhow::{Context, Result};
use aurora_config::dao_config::Settings;

use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, time::SystemTime};
pub mod service;
use service::*;
pub mod models;
#[tokio::main]
async fn main() -> Result<()> {
    //let _addr: SocketAddr = "0.0.0.0:50051".parse()?;
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
fn setup_logger() -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);
    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(log::LevelFilter::Info)
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
