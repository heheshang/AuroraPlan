use crate::{ctx::Ctx, model};
use aurora_common::core_results::results::{ApiResult, Result};
use axum::{
    extract::{Path, Query},
    middleware,
    routing::{delete, get, post, put},
    Form, Router,
};
use log::info;
use tower_cookies::Cookies;

use super::{
    bean::{
        request::tenants::{CreateTenant, TenantListParams, UpdateTenant},
        response::tenants::{Tenant, TenantList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/tenants", get(list).post(create))
        .route("/tenants/:id", put(update).delete(delete_tenant))
        .route("/tenants/verify-code", get(verify_code));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_tenant(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<()>> {
    model::tenants::delete(id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(
    cookies: Cookies,
    ctx: Ctx,
    Path(id): Path<i32>,
    param: Form<UpdateTenant>,
) -> Result<ApiResult<()>> {
    let description = &param.description;
    let queue_id = &param.queue_id;
    model::tenants::update(id, description, queue_id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateTenant>) -> Result<ApiResult<Tenant>> {
    let tenant_code = param.tenant_code.clone();
    let description = param.description.clone();
    let res = model::tenants::create(tenant_code, description, param.queue_id).await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(cookies: Cookies, ctx: Ctx, param: Query<TenantListParams>) -> Result<ApiResult<TenantList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::tenants::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn verify_code(cookies: Cookies, ctx: Ctx, param: Query<CreateTenant>) -> Result<ApiResult<()>> {
    let tenant_code = &param.tenant_code;
    model::tenants::verify_code(tenant_code).await?;
    Ok(ApiResult::build(Some(())))
}
