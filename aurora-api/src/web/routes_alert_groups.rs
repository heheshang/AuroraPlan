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
        request::{
            alert_groups::{CreateAlertGroup, UpdateAlertGroup, VerifyAlertGroup},
            PageParams,
        },
        response::alert_groups::{AlertGroup, AlertGroupList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/alert-groups", get(list).post(create))
        .route(
            "/alert-groups/:id",
            put(update).delete(delete_alert_groups),
        )
        .route("/alert-groups/verify-name", get(verify_name))
        // .route("/alert-groups/list", get(all))
        ;

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_alert_groups(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<()>> {
    model::alert_groups::delete(id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(
    cookies: Cookies,
    ctx: Ctx,
    Path(id): Path<i32>,
    param: Form<UpdateAlertGroup>,
) -> Result<ApiResult<()>> {
    let user_id = ctx.user_id;
    let group_name = &param.group_name;
    let alert_instance_ids = &param.alert_instance_ids;
    let description = &param.description;
    model::alert_groups::update(
        id,
        group_name.clone(),
        alert_instance_ids.clone(),
        description.clone(),
        user_id,
    )
    .await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateAlertGroup>) -> Result<ApiResult<AlertGroup>> {
    let user_id = ctx.user_id;
    let group_name = &param.group_name;
    let alert_instance_ids = &param.alert_instance_ids;
    let description = &param.description;

    let res = model::alert_groups::create(
        group_name.clone(),
        alert_instance_ids.clone(),
        description.clone(),
        user_id,
    )
    .await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(cookies: Cookies, ctx: Ctx, param: Query<PageParams>) -> Result<ApiResult<AlertGroupList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::alert_groups::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn verify_name(cookies: Cookies, ctx: Ctx, param: Query<VerifyAlertGroup>) -> Result<ApiResult<()>> {
    let group_name = &param.group_name;
    model::alert_groups::verify_alert_instance(group_name).await?;
    Ok(ApiResult::build(Some(())))
}
// type ALLList = Vec<AlertGroup>;

// pub async fn all(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<ALLList>> {
//     let data = model::alert_groups::all().await?;
//     Ok(ApiResult::builder().data(data).build())
// }
