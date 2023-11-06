use anyhow::Ok;
use aurora_common::core_results::results::ApiResult;
use aurora_common::core_results::results::Result;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::Json;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::Cookies;

use crate::ctx::Ctx;

use super::mw::mw_auth::mw_ctx_require;

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/define-user-count", get(define_user_count))
        .route_layer(middleware::from_fn(mw_ctx_require));

    Router::new()
        .nest("/aurora/projects/analysis", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

/// #请求信息
/// ```sh
/// curl 'http://pt003:12345/dolphinscheduler/projects/analysis/define-user-count?projectCode=0' \
///   -H 'Accept: application/json, text/plain, */*' \
///   -H 'Accept-Language: zh-CN,zh;q=0.9' \
///   -H 'Cache-Control: no-cache' \
///   -H 'Connection: keep-alive' \
///   -H 'Cookie: sessionId=de87d07b-134f-4b6c-822d-30172e96c1b6; _ga=GA1.1.1260917633.1679362249; csrftoken=cau9yGs64fECWvFjv26ilGb23IG3PNOnCExY5mVIuQtGE4IhuP9DMtpcazyBFw71; language=zh_CN; session=.eJwljztuAzEMRO-iegtSoijRl1nwJySIEQO7dhXk7lkjmGaa9zDzU_Z15PlRbs_jlVvZP6PcihD36jgdhq1gnS7DyAGiQRKvmebc15orq5q6rrQYFPUK98AqA7NpQ3Jk5atT6DSAJm8rDwTs5pMiLCdhY5WqIkHZvBKVrfh5rP35-Mrva8-svceIAQTu9tYxibUOiMpjsAgsE8CLuz9c73kxF7iV15nH_yUsv3-u1UMH.ZTCT8Q.wL9PVkeZK9l0KBAuSwpdk088fnE' \
///   -H 'Pragma: no-cache' \
///   -H 'Referer: http://pt003:12345/dolphinscheduler/ui/home' \
///   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
///   -H 'language: zh_CN' \
///   -H 'sessionId: de87d07b-134f-4b6c-822d-30172e96c1b6' \
///   --compressed \
///   --insecure:w
/// ```
///
/// # 获取用户信息
/// ```json
/// {
///   "code": 0,
///   "msg": "成功",
///   "data": {
///       "count": 227,
///       "userList": [
///           {
///               "userName": "admin",
///               "userId": 1,
///               "count": 227
///           }
///       ]
///   },
///   "failed": false,
///   "success": true
/// }
///
/// ```

pub async fn define_user_count(
    param: Query<DefineUserCountParams>,
    cookies: Cookies,
    ctx: Ctx,
) -> impl IntoResponse {
    let v = json!( {
      "code": 0,
      "msg": "成功",
      "data": {
          "count": 227,
          "userList": [
              {
                  "userName": "admin",
                  "userId": 1,
                  "count": 227
              }
          ]
      },
      "failed": false,
      "success": true
    });
    Json(v)
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DefineUserCountParams {
    projectCode: u32,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TaskStateCountParams {
    projectCode: u32,
    startDate: String,
    endDate: String,
}
///
/// ```sh
/// curl 'http://pt003:12345/dolphinscheduler/projects/analysis/task-state-count?
///
/// startDate=2023-11-06%2000%3A00%3A00&endDate=2023-11-06%2014%3A46%3A54&projectCode=0' \
///   -H 'Accept: application/json, text/plain, */*' \
///   -H 'Accept-Language: zh-CN,zh;q=0.9' \
///   -H 'Cache-Control: no-cache' \
///   -H 'Connection: keep-alive' \
///   -H 'Cookie: sessionId=de87d07b-134f-4b6c-822d-30172e96c1b6; _ga=GA1.1.1260917633.1679362249; csrftoken=cau9yGs64fECWvFjv26ilGb23IG3PNOnCExY5mVIuQtGE4IhuP9DMtpcazyBFw71; language=zh_CN; session=.eJwljztuAzEMRO-iegtSoijRl1nwJySIEQO7dhXk7lkjmGaa9zDzU_Z15PlRbs_jlVvZP6PcihD36jgdhq1gnS7DyAGiQRKvmebc15orq5q6rrQYFPUK98AqA7NpQ3Jk5atT6DSAJm8rDwTs5pMiLCdhY5WqIkHZvBKVrfh5rP35-Mrva8-svceIAQTu9tYxibUOiMpjsAgsE8CLuz9c73kxF7iV15nH_yUsv3-u1UMH.ZTCT8Q.wL9PVkeZK9l0KBAuSwpdk088fnE' \
///   -H 'Pragma: no-cache' \
///   -H 'Referer: http://pt003:12345/dolphinscheduler/ui/home' \
///   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
///   -H 'language: zh_CN' \
///   -H 'sessionId: de87d07b-134f-4b6c-822d-30172e96c1b6' \
///   --compressed \
///   --insecure//
/// ```

/// ```json
/// {
///     "code": 0,
///     "msg": "成功",
///     "data": {
///         "totalCount": 19,
///         "taskCountDtos": [
///             {
///                 "count": 6,
///                 "taskStateType": "SUBMITTED_SUCCESS"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "RUNNING_EXECUTION"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "READY_PAUSE"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "PAUSE"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "READY_STOP"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "STOP"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "FAILURE"
///             },
///             {
///                 "count": 11,
///                 "taskStateType": "SUCCESS"
///             },
///             {
///                 "count": 2,
///                 "taskStateType": "NEED_FAULT_TOLERANCE"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "KILL"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "WAITING_THREAD"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "WAITING_DEPEND"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "DELAY_EXECUTION"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "FORCED_SUCCESS"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "SERIAL_WAIT"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "READY_BLOCK"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "BLOCK"
///             },
///             {
///                 "count": 0,
///                 "taskStateType": "DISPATCH"
///             }
///         ]
///     },
///     "failed": false,
///     "success": true
/// }
///
///
pub async fn task_state_count(
    param: Query<TaskStateCountParams>,
    cookies: Cookies,
    ctx: Ctx,
) -> impl IntoResponse {
    let v = json!( {

    "code": 0,
    "msg": "成功",
    "data": {
        "totalCount": 19,
        "taskCountDtos": [
            {
                "count": 6,
                "taskStateType": "SUBMITTED_SUCCESS"
            },
            {
                "count": 0,
                "taskStateType": "RUNNING_EXECUTION"
            },
            {
                "count": 0,
                "taskStateType": "READY_PAUSE"
            },
            {
                "count": 0,
                "taskStateType": "PAUSE"
            },
            {
                "count": 0,
                "taskStateType": "READY_STOP"
            },
            {
                "count": 0,
                "taskStateType": "STOP"
            },
            {
                "count": 0,
                "taskStateType": "FAILURE"
            },
            {
                "count": 11,
                "taskStateType": "SUCCESS"
            },
            {
                "count": 2,
                "taskStateType": "NEED_FAULT_TOLERANCE"
            },
            {
                "count": 0,
                "taskStateType": "KILL"
            },
            {
                "count": 0,
                "taskStateType": "WAITING_THREAD"
            },
            {
                "count": 0,
                "taskStateType": "WAITING_DEPEND"
            },
            {
                "count": 0,
                "taskStateType": "DELAY_EXECUTION"
            },
            {
                "count": 0,
                "taskStateType": "FORCED_SUCCESS"
            },
            {
                "count": 0,
                "taskStateType": "SERIAL_WAIT"
            },
            {
                "count": 0,
                "taskStateType": "READY_BLOCK"
            },
            {
                "count": 0,
                "taskStateType": "BLOCK"
            },
            {
                "count": 0,
                "taskStateType": "DISPATCH"
            }
        ]
    },
    "failed": false,
    "success": true
    });
    Json(v)
}
