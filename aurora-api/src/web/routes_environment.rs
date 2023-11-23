use crate::web::bean::request::environment::UpdateEnvironment;
use crate::web::bean::response::environment::Environment;
use crate::web::bean::response::environment::EnvironmentList;
use aurora_common::core_results::results::ApiResult;
use aurora_common::core_results::results::Result;
use axum::{
    extract::{Path, Query},
    middleware,
    routing::{get, put},
    Form, Router,
};
use tower_cookies::Cookies;

use crate::{ctx::Ctx, model};

use super::bean::request::environment::VerifyEnvironment;
use super::{
    bean::request::environment::{CreateEnvironment, EnvironmentListParams},
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/environment", get(list).post(create))
        .route("/environment/:id", put(update).delete(delete_environment))
        .route("/environment/verify-environment", get(verify_environment));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_environment(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<()>> {
    model::environment::delete(id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(
    cookies: Cookies,
    ctx: Ctx,
    Path(id): Path<i32>,
    param: Form<UpdateEnvironment>,
) -> Result<ApiResult<()>> {
    let code = param.code;
    let description = param.description.clone();
    let name = &param.name;
    let config = &param.config;
    let worker_groups = param.worker_groups.clone();
    model::environment::update(code, name, config, description, worker_groups).await?;
    Ok(ApiResult::build(Some(())))
}

pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateEnvironment>) -> Result<ApiResult<Environment>> {
    let name = param.name.clone();
    let description = param.description.clone();
    let config = param.config.clone();
    let worker_groups = param.worker_groups.clone();
    let operator = ctx.user_id;
    let res = model::environment::create(name, config, description, worker_groups, operator).await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(
    cookies: Cookies,
    ctx: Ctx,
    param: Query<EnvironmentListParams>,
) -> Result<ApiResult<EnvironmentList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::environment::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn verify_environment(cookies: Cookies, ctx: Ctx, param: Query<VerifyEnvironment>) -> Result<ApiResult<()>> {
    let environment_name = &param.environment_name;
    model::environment::verify_environment(environment_name).await?;
    Ok(ApiResult::build(Some(())))
}
