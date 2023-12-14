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
            cluster::{CreateCluster, DeleteCluster, UpdateCluster, VerifyCluster},
            PageParams,
        },
        response::cluster::{Cluster, ClusterList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/cluster", get(list).post(create).post(delete_cluster)
        .post(update)
    )
        .route("/cluster/verify-cluster", get(verify_name))
        // .route("/cluster/list", get(all))
        ;
    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_cluster(cookies: Cookies, ctx: Ctx, param: Form<DeleteCluster>) -> Result<ApiResult<()>> {
    let code = param.code;
    model::cluster::delete(code).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(cookies: Cookies, ctx: Ctx, param: Form<UpdateCluster>) -> Result<ApiResult<()>> {
    let operator = Some(ctx.user_id);
    let code = param.code;
    let name: Option<String> = Some(param.name.clone());
    let config: Option<String> = Some(param.config.clone());
    let description: Option<String> = param.description.clone();
    model::cluster::update(code, name, config, description.clone(), operator).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn create(cookies: Cookies, ctx: Ctx, param: Form<CreateCluster>) -> Result<ApiResult<Cluster>> {
    let operator = Some(ctx.user_id);
    let name: Option<String> = Some(param.name.clone());
    let config: Option<String> = Some(param.config.clone());
    let description: Option<String> = param.description.clone();

    let res = model::cluster::create(name, config, description, operator).await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(cookies: Cookies, ctx: Ctx, param: Query<PageParams>) -> Result<ApiResult<ClusterList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::cluster::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn verify_name(cookies: Cookies, ctx: Ctx, param: Query<VerifyCluster>) -> Result<ApiResult<()>> {
    let name = &param.cluster_name;
    model::cluster::verify(name).await?;
    Ok(ApiResult::build(Some(())))
}
// type ALLList = Vec<Cluster>;

// pub async fn all(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<ALLList>> {
//     let data = model::cluster::all().await?;
//     Ok(ApiResult::builder().data(data).build())
// }
