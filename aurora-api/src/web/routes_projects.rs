use aurora_common::core_results::results::{ApiResult, Result};
use axum::{
    extract::Json,
    extract::{Path, Query},
    middleware,
    response::IntoResponse,
    routing::{get, patch, post},
    Form, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::Cookies;
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        self,
        projects::service::{_create_project_paramter, _project_parameter_list, create},
    },
};
use crate::{model::projects::service::list, web::bean::response::projects::DsProjectRes};

use super::{
    bean::{
        request::projects::{
            DefineUserCountParams, ProcessStateCountParams, ProjectCreateParams, ProjectListParams,
            ProjectParamCreate, ProjectParameterListParams, TaskStateCountParams,
        },
        response::projects::{DsProjectList, DsProjectParamterRes, ProjectParameterList},
    },
    mw::mw_auth::mw_ctx_require,
};

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
        .route("/projects", get(project_list).post(create_project))
        .route(
            "/projects/:project_code/project-parameter",
            post(create_project_parameter).get(project_parameter_list),
        );

    Router::new()
        .nest("/aurora", routes)
        .route_layer(middleware::from_fn(mw_ctx_require))
}
pub async fn project_parameter_list(
    cookies: Cookies,
    ctx: Ctx,
    Path(project_code): Path<u64>,
    param: Query<ProjectParameterListParams>,
) -> Result<ApiResult<ProjectParameterList>> {
    let res = _project_parameter_list(
        &param.pageNo,
        &param.pageSize,
        &param.searchVal,
        &project_code,
    )
    .await?;
    Ok(ApiResult::build(Some(ProjectParameterList::from(res))))
}
pub async fn create_project_parameter(
    cookies: Cookies,
    ctx: Ctx,
    Path(project_code): Path<i64>,
    param: Form<ProjectParamCreate>,
) -> Result<ApiResult<DsProjectParamterRes>> {
    info!("projectCode: {:?} ,param :{:#?} ", project_code, param);
    let user_id = ctx.user_id;
    let project_parameter_name = param.projectParameterName.clone();
    let project_parameter_value = param.projectParameterValue.clone();
    let res = _create_project_paramter(
        user_id,
        project_code,
        project_parameter_name,
        project_parameter_value,
    )
    .await?;
    Ok(ApiResult::build(Some(DsProjectParamterRes::from(res))))
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
) -> Result<ApiResult<DsProjectList>> {
    let res = list(&param.pageNo, &param.pageSize, &param.searchVal).await?;
    Ok(ApiResult::build(Some(res)))
}
