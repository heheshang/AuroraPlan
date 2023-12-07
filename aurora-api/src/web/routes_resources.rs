use std::{collections::HashMap, net::SocketAddr};

use crate::{ctx::Ctx, cypt::security::get_authenticator, model, web, web::bean::request::user::UserLoginInfoReq};
use aurora_common::{
    core_error::error::Error,
    core_results::results::{ApiResult, Result},
};
use axum::{
    extract::{ConnectInfo, Query},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Form, Json, Router,
};
use log::error;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::Cookies;

use super::{
    bean::{request::user::UserInfoReq, response::user::UserInfoRes},
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new().route("/resources/base-dir", get(base_dir));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct BaseDirParams {
    // #[serde(rename = "type")]
    r#type: String,
}
pub async fn base_dir(param: Query<BaseDirParams>, cookies: Cookies, ctx: Ctx) -> impl IntoResponse {
    let res = json!(
        {
    "code": 0,
    "msg": "成功",
    "data": "file:/dolphinscheduler",
    "failed": false,
    "success": true

    });
    Json(res)
}
