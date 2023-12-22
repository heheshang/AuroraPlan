use crate::web::bean::request::environment::UpdateEnvironment;
use crate::web::bean::response::environment::Environment;
use crate::web::bean::response::environment::EnvironmentList;
use axum::routing::delete;
use axum::routing::post;
use axum::Json;
use axum::{
    extract::{Path, Query},
    middleware,
    routing::{get, put},
    Form, Router,
};
use lib_common::core_results::results::ApiResult;
use lib_common::core_results::results::Result;
use tower_cookies::Cookies;
use tracing::info;

use crate::{ctx::Ctx, model};

use super::bean::request::environment::VerifyEnvironment;
use super::bean::response::environment::EnvironmentPage;
use super::{
    bean::request::environment::{CreateEnvironment, EnvironmentListParams},
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/environment/create", post(create))
        .route("/environment/list-paging", get(list))
        .route("/environment/query-environment-list", get(query_environment_list))
        .route("/environment/update", post(update))
        .route("/environment/delete", delete(delete_environment))
        .route("/environment/verify-environment", post(verify_environment));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_environment(
    cookies: Cookies,
    ctx: Ctx,
    #[allow(non_snake_case)] environmentCode: Form<i64>,
) -> Result<ApiResult<()>> {
    model::environment::delete(environmentCode.0).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(cookies: Cookies, ctx: Ctx, param: Form<UpdateEnvironment>) -> Result<ApiResult<()>> {
    let code = param.code;
    let description = param.description.clone();
    let name = &param.name;
    let config = &param.config;
    let mut origin = param.worker_groups.clone();
    let worker_groups = origin
        .replace(['[', ']', '\"'], "")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    info!(
        "name: {},description: {:?} config: {},worker_groups: {:?}  ",
        name, description, config, worker_groups
    );
    model::environment::update(code, name, config, description, worker_groups).await?;
    Ok(ApiResult::build(Some(())))
}

pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateEnvironment>) -> Result<ApiResult<EnvironmentPage>> {
    let name = param.name.clone();
    let description = param.description.clone();
    let config = param.config.clone();
    //"[\"222\"]"; convert to vec
    // let mut worker_groups = param.worker_groups.clone();
    // worker_groups.remove_matches('[');
    // worker_groups.remove_matches(']');
    // worker_groups.replace("\"", "");
    // worker_groups.replace(" ", "");
    let mut origin = param.worker_groups.clone();
    let worker_groups = origin
        .replace(['[', ']', '\"'], "")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let operator = ctx.user_id;
    info!(
        "name: {},description: {:?} config: {},worker_groups: {:?},operator: {} ",
        name, description, config, worker_groups, operator
    );
    let res = model::environment::create(name, config, description, worker_groups.to_vec(), operator).await?;
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
pub async fn query_environment_list(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<Vec<EnvironmentPage>>> {
    let res = model::environment::all().await?;
    Ok(ApiResult::build(Some(res)))
}
pub async fn verify_environment(cookies: Cookies, ctx: Ctx, param: Form<VerifyEnvironment>) -> Result<ApiResult<()>> {
    let environment_name = &param.environment_name;
    model::environment::verify_environment(environment_name).await?;
    Ok(ApiResult::build(Some(())))
}
