mod ctx;
mod cypt;
mod model;
mod utils;
mod web;
use anyhow::Result;
use lib_common::logger::setup_logger;
use lib_conifg::api_config::Settings;
// use tracing::info;
use std::{env, net::SocketAddr};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    setup_logger()?;
    info!("log init success!");
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;
    // let _ = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .thread_name("aurora-api")
    //     // .thread_stack_size(32 * 100 * 1024 * 1024)
    //     .build()
    //     .unwrap()
    //     .block_on(start(host, port));
    info!("start aurora-api");
    start(host, port).await?;
    Ok(())
}

async fn start(host: String, port: u32) -> Result<()> {
    // let backtrace = backtrace::Backtrace::new();
    info!("start aurora-api");
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("{:<12}->{}", "listen", addr);

    let route_all = web::route_all().await;
    let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
    let make_service = route_all.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(tcp_listener, make_service).await?;

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
