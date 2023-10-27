use std::{collections::HashMap, net::SocketAddr};

use crate::{
    ctx::Ctx, cypt::security::get_authenticator, log, model, web,
    web::bean::request::ds_user_req::UserLoginInfoReq,
};
use aurora_common::{
    core_error::error::Error,
    core_results::results::{ApiResult, Result},
};
use axum::{
    extract::ConnectInfo,
    middleware,
    routing::{get, post},
    Json, Router,
};
use tower_cookies::Cookies;
use tracing::error;

use super::{
    bean::{request::ds_user_req::UserInfoReq, response::ds_user_res::UserInfoRes},
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/logout", post(logout))
        .route("/get_user_info", get(user_info));
    let login = Router::new().route("/login", post(login));

    let other = Router::new()
        .nest("/aurorascheduler", routes)
        .route_layer(middleware::from_fn(mw_ctx_require));
    Router::new().nest("/aurorascheduler", login).merge(other)
}

pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    cookies: Cookies,
    Json(payload): Json<UserLoginInfoReq>,
) -> Result<ApiResult<HashMap<String, String>>> {
    let user_name = payload.user_name.clone();
    let user_password = payload.user_password.clone();
    if user_name.is_empty() {
        return Err(Error::UserNamePasswdError);
    }
    // let ip = "127.0.0.1".to_string();
    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return Err(Error::IpIsEmpty);
    }
    get_authenticator()
        .authenticate(user_name, user_password, ip)
        .await
        .map(|res| {
            web::set_token_cookie(&cookies, &res["sessionId"]).unwrap();
            ApiResult::build(Some(res))
        })
        .map_err(|e| {
            error!("api_login_handler error: {:?}", e);
            e
        })
}
pub async fn logout() {}

pub async fn user_info(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<UserInfoRes>> {
    let user_id = ctx.user_id();
    let user = model::user::service::_get_user(user_id).await?;
    Ok(ApiResult::build(Some(UserInfoRes::from(user))))
}
