use aurora_common::core_results::results::{ApiResult, Result};
use axum::{
    extract::Json,
    extract::Query,
    middleware,
    response::IntoResponse,
    routing::{get, patch, post},
    Form, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::Cookies;

use crate::web::bean::response::arurora_projects_res::DsProjectRes;
use crate::{
    ctx::Ctx,
    model::{self, projects::service::create},
};

use super::bean::request::arurora_projects_req::{
    DefineUserCountParams, ProcessStateCountParams, ProjectCreateParams, ProjectListParams,
    TaskStateCountParams,
};
use super::mw::mw_auth::mw_ctx_require;

pub fn routes() -> Router {
    let routes = Router::new()
        .route(
            "/projects/analysis/define-user-count",
            get(define_user_count),
        )
        .route("/projects/analysis/task-state-count", get(task_state_count))
        .route(
            "/projects/analysis/process-state-count",
            get(process_state_count),
        )
        .route("/projects", get(project_list).post(create_project));

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

pub async fn create_project(
    cookies: Cookies,
    ctx: Ctx,
    param: Form<ProjectCreateParams>,
) -> Result<ApiResult<DsProjectRes>> {
    let user_id = ctx.user_id;
    let name = param.projectName.clone();
    let description = param.description.clone();
    let res = create(user_id, name, description).await?;
    Ok(ApiResult::build(Some(DsProjectRes::from(res))))
}

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
pub async fn process_state_count(
    param: Query<ProcessStateCountParams>,
    cookies: Cookies,
    ctx: Ctx,
) -> impl IntoResponse {
    let res = json!({
    "code": 0,
    "msg": "成功",
    "data": {
        "totalCount": 22,
        "taskCountDtos": [
            {
                "count": 0,
                "taskStateType": "SUBMITTED_SUCCESS"
            },
            {
                "count": 12,
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
                "count": 10,
                "taskStateType": "SUCCESS"
            },
            {
                "count": 0,
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
    Json(res)
}
pub async fn project_list(
    param: Query<ProjectListParams>,
    cookies: Cookies,
    ctx: Ctx,
) -> impl IntoResponse {
    let res = json!({
        "code": 0,
        "msg": "成功",
        "data": {
            "totalList": [
                {
                    "id": 3,
                    "userId": 1,
                    "userName": "admin",
                    "code": "8721014310784",
                    "name": "test",
                    "description": "",
                    "createTime": "2023-02-28 13:48:44",
                    "updateTime": "2023-02-28 13:48:44",
                    "perm": 0,
                    "defCount": 0,
                    "instRunningCount": 0
                },
                {
                    "id": 2,
                    "userId": 1,
                    "userName": "admin",
                    "code": "8442776973696",
                    "name": "dolphinscheduler",
                    "description": "",
                    "createTime": "2023-02-03 09:59:55",
                    "updateTime": "2023-02-03 09:59:55",
                    "perm": 0,
                    "defCount": 227,
                    "instRunningCount": 30
                }
            ],
            "total": 2,
            "totalPage": 1,
            "pageSize": 10,
            "currentPage": 1,
            "start": 0
        },
        "failed": false,
        "success": true
    });
    Json(res)
}
