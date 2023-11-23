mod ctx;
mod cypt;
mod log;
mod model;
mod utils;
mod web;
use anyhow::Result;
use aurora_config::api_config::Settings;
use std::{env, net::SocketAddr};
use tracing::{info, Level};

fn main() -> Result<()> {
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("aurora-api")
        // .thread_stack_size(32 * 100 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(start(host, port));
    Ok(())
}

async fn start(host: String, port: u32) -> Result<()> {
    // let backtrace = backtrace::Backtrace::new();
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_ansi(true)
        .with_line_number(true)
        // .with_file(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let route_all = web::route_all().await;
    axum::Server::bind(&addr)
        .serve(route_all.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    info!("log init success!");
    info!("{:<12}->{}", "listen", addr);
    // println!("log init success! {:#?}", backtrace);
    Ok(())
}
// fn using_serve_dir() -> Router {
//     // serve the file in the "assets" directory under `/assets`
//     let path = get_ui_source_path();
//     match path {
//         Ok(p) => Router::new().route_service(
//             "/aurora/ui/*rest",
//             ServeDir::new(p.to_str().unwrap()).append_index_html_on_directories(true),
//         ),

//         Err(_) => Router::new(),
//     }
// }
