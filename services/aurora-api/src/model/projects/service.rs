use crate::{
    model::client::service::{_ds_project_parameter_service_client, _ds_project_service_client},
    web::bean::response::projects::DsProjectList,
};
use lib_common::{core_error::error::Error, core_results::results::Result};
use lib_proto::{
    ds_project::{DeleteDsProjectRequest, DsProject, ListDsProjectsRequest, UpdateDsProjectRequest},
    ds_project_parameter::{
        CreateProjectParameterRequest, DeleteProjectParameterRequest, ListProjectParametersRequest,
        ListProjectParametersResponse, ProjectParameter, UpdateProjectParameterRequest,
    },
    pb::ds_project::CreateDsProjectRequest,
};
use tracing::error;

pub async fn create(user_id: i32, name: String, description: Option<String>) -> Result<DsProject> {
    let client = _ds_project_service_client().await?;
    let request = tonic::Request::new(CreateDsProjectRequest {
        name,
        user_id,
        description,
    });
    client
        .clone()
        .create_ds_project(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create_project error: {:?}", err);
            err
        })
}
pub async fn update(user_id: i32, name: String, description: Option<String>) -> Result<DsProject> {
    let client = _ds_project_service_client().await?;
    let request = tonic::Request::new(UpdateDsProjectRequest {
        name,
        user_id,
        description,
    });
    client
        .clone()
        .update_ds_project(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update_project error: {:?}", err);
            err
        })
}

pub async fn _delete_project(project_code: i32) -> Result<()> {
    let client = _ds_project_service_client().await?;
    let request = tonic::Request::new(DeleteDsProjectRequest { id: project_code });
    client.clone().delete_ds_project(request).await.map_err(|e| {
        let err: Error = e.into();
        error!("update_project error: {:?}", err);
        err
    })?;
    Ok(())
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<DsProjectList> {
    let client = _ds_project_service_client().await?;
    let request = tonic::Request::new(ListDsProjectsRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });

    Ok(client
        .clone()
        .list_ds_projects(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list_project error: {:?}", err);
            err
        })?
        .into())
}

pub async fn _create_project_paramter(
    user_id: i32,
    project_code: i64,
    project_parameter_name: String,
    project_parameter_value: String,
) -> Result<ProjectParameter> {
    let client = _ds_project_parameter_service_client().await?;
    let request = tonic::Request::new(CreateProjectParameterRequest {
        project_parameter: Some(ProjectParameter {
            user_id: Some(user_id),
            project_code,
            param_name: project_parameter_name,
            param_value: project_parameter_value,
            ..Default::default()
        }),
    });

    client
        .clone()
        .create_project_parameter(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create_project_paramter error: {:?}", err);
            err
        })
}
pub async fn _delete_project_parameter(code: i64, project_code: i64) -> Result<()> {
    let client = _ds_project_parameter_service_client().await?;
    let request = tonic::Request::new(DeleteProjectParameterRequest { code, project_code });

    client
        .clone()
        .delete_project_parameter(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete project parameter error: {:?}", err);
            err
        })
}
pub async fn _update_project_parameter(
    code: i64,
    project_code: i64,
    project_parameter_name: String,
    project_parameter_value: String,
) -> Result<ProjectParameter> {
    let client = _ds_project_parameter_service_client().await?;
    let request = tonic::Request::new(UpdateProjectParameterRequest {
        code,
        project_code,
        param_name: project_parameter_name,
        param_value: project_parameter_value,
    });
    client
        .clone()
        .update_project_parameter(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update project paramter error: {:?}", err);
            err
        })
}
pub async fn _project_parameter_list(
    page_num: &i64,
    page_size: &i64,
    search_val: &Option<String>,
    project_code: &i64,
) -> Result<ListProjectParametersResponse> {
    let client = _ds_project_parameter_service_client().await?;
    let request = tonic::Request::new(ListProjectParametersRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
        project_code: *project_code,
    });
    client
        .clone()
        .list_project_parameters(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list_project_parameter error: {:?}", err);
            err
        })
}
