use crate::ctx::Ctx;
use crate::model;

use super::bean::request::ui_plugin::UiPluginQueryByType;
use super::bean::response::ui_plugin::UiPlugin;
use super::bean::response::ui_plugin::UiPluginNoParams;
use super::mw::mw_auth::mw_ctx_require;
use axum::extract::Path;
use axum::extract::Query;
use axum::routing::get;
use axum::{middleware, Router};
use lib_common::core_results::results::ApiResultBuilder;
use lib_common::core_results::results::{ApiResult, Result};
use tower_cookies::Cookies;

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/ui-plugins/:id", get(ui_plugin))
        .route("/ui-plugins/query-by-type", get(query_by_type));
    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

async fn ui_plugin(cookies: Cookies, ctx: Ctx, Path(id): Path<i32>) -> Result<ApiResult<UiPlugin>> {
    Ok(ApiResult::builder().data(model::ui_plugins::get_ui_plugins(id).await?).build())
}
async fn query_by_type(
    cookies: Cookies,
    ctx: Ctx,
    payload: Query<UiPluginQueryByType>,
) -> Result<ApiResult<Vec<UiPluginNoParams>>> {
    Ok(ApiResult::builder()
        .data(model::ui_plugins::get_ui_by_type(&payload.plugin_type).await?)
        .build())
}
