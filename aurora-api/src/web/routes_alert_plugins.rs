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
        request::{
            alert_plugin_instances::{CreateAlertPluginInstance, UpdateAlertPluginInstance, VerifyAlertPluginInstance},
            PageParams,
        },
        response::alert_plugin_instances::{AlertPluginInstance, AlertPluginInstanceList},
    },
    mw::mw_auth::mw_ctx_require,
};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/alert-plugin-instances", get(list).post(create))
        .route(
            "/alert-plugin-instances/:id",
            put(update).delete(delete_alert_plugin_instance),
        )
        .route("/alert-plugin-instances/verify-name", get(verify_name))
        .route("/alert-plugin-instances/list", get(all));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn delete_alert_plugin_instance(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<()>> {
    model::alert_plugin_instances::delete(id).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn update(
    cookies: Cookies,
    ctx: Ctx,
    Path(id): Path<i32>,
    param: Form<UpdateAlertPluginInstance>,
) -> Result<ApiResult<()>> {
    let instance_name = &param.instance_name;
    let plugin_instance_params = &param.plugin_instance_params;
    let alert_plugin_instance_id = param.alert_plugin_instance_id;
    model::alert_plugin_instances::update(id, instance_name.to_string(), plugin_instance_params.to_string()).await?;
    Ok(ApiResult::build(Some(())))
}
pub async fn create(
    cookies: Cookies,
    ctx: Ctx,
    param: Form<CreateAlertPluginInstance>,
) -> Result<ApiResult<AlertPluginInstance>> {
    let instance_name = &param.instance_name;
    let plugin_instance_params = &param.plugin_instance_params;
    let alert_plugin_instance_id = param.plugin_define_id;
    let res = model::alert_plugin_instances::create(
        instance_name.to_string(),
        alert_plugin_instance_id,
        plugin_instance_params.to_string(),
    )
    .await?;
    Ok(ApiResult::build(Some(res)))
}

pub async fn list(cookies: Cookies, ctx: Ctx, param: Query<PageParams>) -> Result<ApiResult<AlertPluginInstanceList>> {
    let page_num = param.pageNo;
    let page_size = param.pageSize;
    let search_val = &param.searchVal;
    let res = model::alert_plugin_instances::list(&page_num, &page_size, search_val).await?;
    Ok(ApiResult::build(Some(res.into())))
}

pub async fn verify_name(cookies: Cookies, ctx: Ctx, param: Query<VerifyAlertPluginInstance>) -> Result<ApiResult<()>> {
    let instance_name = &param.alert_instance_name;
    model::alert_plugin_instances::verify_alert_instance(instance_name).await?;
    Ok(ApiResult::build(Some(())))
}
type ALLList = Vec<AlertPluginInstance>;

pub async fn all(cookies: Cookies, ctx: Ctx) -> Result<ApiResult<ALLList>> {
    let data = model::alert_plugin_instances::all().await?;
    Ok(ApiResult::builder().data(data).build())
}
