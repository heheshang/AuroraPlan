#![allow(unused)]
pub(crate) mod bean;
pub(crate) mod mw;
pub(crate) mod routes_alert_groups;
pub(crate) mod routes_alert_plugins;
pub(crate) mod routes_cluster;
pub(crate) mod routes_environment;
pub(crate) mod routes_projects;
pub(crate) mod routes_queues;
pub(crate) mod routes_resources;
pub(crate) mod routes_tenants;
pub(crate) mod routes_ui_plugins;
pub(crate) mod routes_user;
pub(crate) mod routes_worker_groups;

// region:    --- Modules

use aurora_common::core_results::results::Result;
use axum::{middleware, Router};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use self::mw::{mw_auth::mw_ctx_resolve, mw_req_map::log_path_params, mw_res_map::mw_response_map};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "sessionId";

fn set_token_cookie(cookies: &Cookies, session: &str) -> Result<()> {
    // let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, session.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

pub async fn route_all() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            // .merge(using_serve_dir())
            .merge(routes_user::routes())
            .merge(routes_projects::routes())
            .merge(routes_resources::routes())
            .merge(routes_queues::routes())
            .merge(routes_tenants::routes())
            .merge(routes_environment::routes())
            .merge(routes_worker_groups::routes())
            .merge(routes_ui_plugins::routes())
            .merge(routes_alert_plugins::routes())
            .merge(routes_alert_groups::routes())
            .merge(routes_cluster::routes())
            // .route("/aurora/ui/home", get(hello))
            .layer(middleware::map_request(log_path_params))
            .layer(middleware::map_response(mw_response_map))
            .layer(middleware::from_fn(mw_ctx_resolve))
            .layer(CookieManagerLayer::new()),
    )
}
