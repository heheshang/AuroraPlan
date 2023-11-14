use crate::model::client::service::_ds_project_service_client;
use crate::web::bean::response::arurora_projects_res::DsProjectList;
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::{
    ds_project::{DsProject, ListDsProjectsRequest},
    pb::ds_project::CreateDsProjectRequest,
};
use tracing::log::error;

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

pub async fn list(
    page_num: &u64,
    page_size: &u64,
    search_val: &Option<String>,
) -> Result<DsProjectList> {
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
