use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_environment::{
    CreateDsEnvironmentRequest, DeleteDsEnvironmentRequest, ListDsEnvironmentsRequest, ListDsEnvironmentsResponse,
    UpdateDsEnvironmentRequest, VerifyDsEnvironmentRequest,
};
use log::{error, info};

use crate::web::bean::response::environment::EnvironmentPage;

use super::client::service::_ds_environment_service_client;

pub async fn delete(code: i64) -> Result<()> {
    let client = _ds_environment_service_client().await?;
    let request = tonic::Request::new(DeleteDsEnvironmentRequest { code });
    client
        .clone()
        .delete_ds_environment(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete environments error: {:?}", err);
            err
        })?;
    Ok(())
}

pub async fn create(
    name: String,
    config: String,
    description: Option<String>,
    worker_groups: Vec<String>,
    operator: i32,
) -> Result<EnvironmentPage> {
    let client = _ds_environment_service_client().await?;
    let request = tonic::Request::new(CreateDsEnvironmentRequest {
        name,
        config,
        description,
        worker_groups,
        operator,
    });
    let res = client
        .clone()
        .create_ds_environment(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create environments error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn verify_environment(environment_name: &str) -> Result<()> {
    let client = _ds_environment_service_client().await?;
    let request = tonic::Request::new(VerifyDsEnvironmentRequest {
        environment_name: environment_name.to_string(),
    });
    client.clone().verify_ds_environment(request).await?;
    Ok(())
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsEnvironmentsResponse> {
    info!(
        "list environments page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_environment_service_client().await?;
    let request = tonic::Request::new(ListDsEnvironmentsRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_environments(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list environments error: {:?}", err);
            err
        })?;
    Ok(res)
}

pub async fn update(
    code: i64,
    name: &str,
    config: &str,
    description: Option<String>,
    worker_groups: Vec<String>,
) -> Result<()> {
    let client = _ds_environment_service_client().await?;
    let request = tonic::Request::new(UpdateDsEnvironmentRequest {
        code,
        name: Some(name.to_string()),
        config: Some(config.to_string()),
        description,
        worker_groups,
    });
    client
        .clone()
        .update_ds_environment(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update environments error: {:?}", err);
            err
        })?;
    Ok(())
}
