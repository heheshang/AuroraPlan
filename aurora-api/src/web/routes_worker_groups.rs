use crate::{ctx::Ctx, model};
use aurora_common::core_results::results::{ApiResult, Result};
use axum::{
    extract::{Path, Query},
    middleware,
    routing::{delete, get, post, put},
    Form, Router,
};
use tower_cookies::Cookies;
use tracing::info;

use super::{
    bean::{
        request::worker_groups::{CreateWorkerGroups, UpdateWorkerGroups, WorkerGroupsListParams},
        response::worker_groups::{WorkerGroups, WorkerGroupsList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/worker-groups", get(list).post(create))
        .route("/worker-groups/worker-address-list", get(worker_address_list))
        .route("/worker-groups/:id", put(update).delete(delete_tenant))
        .route("/worker-groups/all", get(all));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_tenant(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<()>> {
    model::worker_groups::delete(id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(
    cookies: Cookies,
    ctx: Ctx,
    Path(id): Path<i64>,
    param: Form<UpdateWorkerGroups>,
) -> Result<ApiResult<()>> {
    let name = &param.name;
    let addr_list = &param.addr_list;
    model::worker_groups::update(id, name, addr_list).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateWorkerGroups>) -> Result<ApiResult<WorkerGroups>> {
    let name = param.name.clone();
    let addr_list = param.addr_list.clone();
    let res = model::worker_groups::create(name, addr_list).await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(
    cookies: Cookies,
    ctx: Ctx,
    param: Query<WorkerGroupsListParams>,
) -> Result<ApiResult<WorkerGroupsList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::worker_groups::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn worker_address_list(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<Vec<String>>> {
    Ok(ApiResult::build(Some(vec!["127.0.0.1:12345".to_string()])))
}

pub async fn all(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<Vec<String>>> {
    let res = model::worker_groups::all().await?;
    Ok(ApiResult::build(Some(res)))
}
