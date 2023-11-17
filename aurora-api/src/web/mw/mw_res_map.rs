use aurora_common::core_error::error::AuroraErrorInfo;
use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use tracing::error;
use tracing::info;

pub async fn mw_response_map(
    // ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    info!("{:<12} - mw_response_map {uri:?} {req_method:?} {res:?}", "RES_MAPPER");
    // let body = res.body();
    let web_error = res.extensions().get::<AuroraErrorInfo>();
    info!("web_error: {:?}", web_error);
    let error_response = web_error.as_ref().map(|error| {
        error!("error: {:?}", error);
        let status_code = match error.code {
            10010 | 10013 | 10014 => StatusCode::UNAUTHORIZED,
            10000 => StatusCode::OK,
            _ => StatusCode::OK,
        };
        (
            status_code,
            Json(json!({
                  "data":error.error_data ,
                  "failed":true,
                  "success":false,
                  "code":error.code,
                  "msg":error.cn_msg

            })),
        )
            .into_response()
    });

    // -- Build and log the server log line.

    error_response.unwrap_or(res)
    // info!("body: {:?}", body);
}
