use std::{collections::HashMap, net::SocketAddr};

use crate::{ctx::Ctx, cypt::security::get_authenticator, model, web, web::bean::request::user::UserLoginInfoReq};
use axum::{
    extract::{ConnectInfo, Query},
    middleware,
    routing::{get, post},
    Form, Json, Router,
};
use lib_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::{ApiResult, Result},
};
use tower_cookies::Cookies;
use tracing::error;

use super::{
    bean::{
        request::{
            user::{UserInfo, UserInfoReq},
            PageParams,
        },
        response::user::{UserInfoRes, UserList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/logout", post(logout))
        .route("/users/list-paging", get(list_paging))
        .route("/users/verify-user-name", get(verify_user_name))
        .route("/users/update", post(update_user))
        .route("/users/create", post(create_user))
        .route("/users/get-user-info", get(user_info));
    let login = Router::new().route("/login", post(login));

    let other = Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require));
    Router::new().nest("/aurora", login).merge(other)
}

pub async fn verify_user_name(_ctx: Ctx, payload: Query<UserInfoReq>) -> Result<ApiResult<()>> {
    model::user::service::verify_user_name(payload.user_name.clone()).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update_user(_ctx: Ctx, Form(payload): Form<UserInfo>) -> Result<ApiResult<()>> {
    model::user::service::update(payload).await?;
    Ok(ApiResult::build(Some(())))
}

pub async fn create_user(_ctx: Ctx, Form(payload): Form<UserInfo>) -> Result<ApiResult<UserInfoRes>> {
    let res = model::user::service::create(payload).await?;
    Ok(ApiResult::build(Some(res)))
}
pub async fn list_paging(_ctx: Ctx, payload: Query<PageParams>) -> Result<ApiResult<UserList>> {
    let res = model::user::service::list(&payload.pageNo, &payload.pageSize, payload.searchVal.clone()).await?;
    Ok(ApiResult::build(Some(UserList::from(res))))
}

pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    cookies: Cookies,
    Form(payload): Form<UserLoginInfoReq>,
) -> Result<ApiResult<HashMap<String, String>>> {
    let user_name = payload.user_name.clone();
    let user_password = payload.user_password.clone();
    if user_name.is_empty() {
        return Err(Error::UserNamePasswdError(AuroraData::Null, None));
    }
    // let ip = "127.0.0.1".to_string();
    let ip = addr.ip().to_string();
    if ip.is_empty() {
        return Err(Error::IpIsEmpty(AuroraData::Null, None));
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
