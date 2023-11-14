mod ctx;
pub mod cypt;
mod log;
pub mod model;
mod utils;
mod web;

use crate::web::mw::{
    mw_auth::mw_ctx_resolve, mw_req_map::log_path_params, mw_res_map::mw_response_map,
};
use anyhow::Result;
use aurora_config::api_config::Settings;

use axum::{middleware, Router};
use std::{env, net::SocketAddr};
use tower_cookies::CookieManagerLayer;
use tracing::{info, Level};
use web::routes_projects;
use web::routes_resources;
use web::routes_user;

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
        .with_ansi(true)
        .with_line_number(true)
        // .with_file(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("log init success!");
    info!("{:<12}->{}", "listen", addr);

    let route_all = Router::new().nest("/api",
        Router::new()
        // .merge(using_serve_dir())
        .merge(routes_user::routes())
        .merge(routes_projects::routes())
        .merge(routes_resources::routes())
        // .route("/aurora/ui/home", get(hello))
        .layer(middleware::map_request(log_path_params))
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn(mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
)
        // .fallback(get_service(StaticFiles::new("./static/")))
        // .merge(using_serve_dir())
        ;

    axum::Server::bind(&addr)
        .serve(route_all.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

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
