use crate::{ctx::Ctx, model};
use axum::{
    extract::{Path, Query},
    middleware,
    routing::{delete, get, post},
    Form, Router,
};
use lib_common::core_results::results::{ApiResult, Result};
use tower_cookies::Cookies;
use tracing::info;

use super::{
    bean::{
        request::queue::{self, CreateQueueRequest, QueueListParams, UpdateQueue},
        response::queue::{Queue, QueueList, VerifyQueue},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/queues", get(list).post(create))
        .route("/queues/:id", delete(delete_queue).put(update_queue))
        .route("/queues/list", get(list_all_queue))
        .route("/queues/verify", post(verify));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn list_all_queue(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<Vec<Queue>>> {
    let res = model::queues::list_all_queue().await?;
    Ok(ApiResult::build(Some(res)))
}
pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateQueueRequest>) -> Result<ApiResult<Queue>> {
    let queue = &param.queue;
    let queue_name = &param.queue_name;
    let res = model::queues::create(queue, queue_name).await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(cookies: Cookies, ctx: Ctx, param: Query<QueueListParams>) -> Result<ApiResult<QueueList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::queues::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}
/// 队列验证
pub async fn verify(cookies: Cookies, ctx: Ctx, param: Form<CreateQueueRequest>) -> Result<ApiResult<VerifyQueue>> {
    let queue = &param.queue;
    let queue_name = &param.queue_name;
    let res = model::queues::verify(queue, queue_name).await?;
    Ok(ApiResult::build(Some(res)))
}

/// delete
pub async fn delete_queue(cookies: Cookies, Path(id): Path<i32>, ctx: Ctx) -> Result<ApiResult<()>> {
    Ok(ApiResult::build(Some(model::queues::delete(id).await?)))
}

pub async fn update_queue(
    cookies: Cookies,
    Path(id): Path<i32>,
    ctx: Ctx,
    param: Form<UpdateQueue>,
) -> Result<ApiResult<Queue>> {
    info!(
        "update_queue: id: {}, queue: {:?}, queue_name: {:?}",
        id, param.queue, param.queue_name
    );
    let queue = &param.queue;
    let queue_name = &param.queue_name;
    Ok(ApiResult::build(Some(
        model::queues::update(id, queue, queue_name).await?,
    )))
}
