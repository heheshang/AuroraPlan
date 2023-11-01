mod ctx;
mod cypt;
mod log;
mod model;
mod utils;
mod web;

mod tests;
use crate::web::mw::{
    mw_auth::mw_ctx_resolve, mw_req_map::log_path_params, mw_res_map::mw_response_map,
};
use anyhow::Result;
use aurora_config::api_config::Settings;
use axum::{middleware, routing::get, Router};
use std::{env, net::SocketAddr};
use tower_cookies::CookieManagerLayer;
use tracing::{info, Level};
use web::routes_user;
async fn hello() -> &'static str {
    info!("hello world");
    "hello world"
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        // .with_file(true)
        .with_line_number(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("log init success!");
    info!("{:<12}->{}", "listen", addr);

    let route_all = Router::new()
        .merge(routes_user::routes())
        .route("/api", get(hello))
        .layer(middleware::map_request(log_path_params))
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn(mw_ctx_resolve))
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&addr)
        .serve(route_all.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
