use aurora_config::get_ui_source_path;
use axum::{extract::Path, http::Request, middleware::map_request, response::Redirect, routing::get, Router};
use std::collections::HashMap;
use tracing::info;
pub async fn log_path_params<B>(Path(path_params): Path<HashMap<String, String>>, request: Request<B>) -> Request<B> {
    info!("uri: {:?}", request.uri());
    info!("method: {:?}", request.method());
    info!("headers: {:?}", request.headers());

    request
}
