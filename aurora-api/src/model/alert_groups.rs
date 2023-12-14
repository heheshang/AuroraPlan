use crate::{model::client::service::_ds_alert_group_service_client, web::bean::response::alert_groups::AlertGroup};
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_alertgroup::{
    CreateDsAlertGroupRequest, DeleteDsAlertGroupRequest, ListDsAlertGroupsRequest, ListDsAlertGroupsResponse,
    UpdateDsAlertGroupRequest, VerifyAlertGroupRequest,
};
use log::{error, info};

pub async fn delete(id: i32) -> Result<()> {
    let client = _ds_alert_group_service_client().await?;
    let request = tonic::Request::new(DeleteDsAlertGroupRequest { id });
    client
        .clone()
        .delete_ds_alert_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete alet groups error: {:?}", err);
            err
        })?;
    Ok(())
}

pub async fn create(
    group_name: String,
    alert_plugin_instance_ids: String,
    description: Option<String>,
    user_id: i32,
) -> Result<AlertGroup> {
    let client = _ds_alert_group_service_client().await?;
    let request = tonic::Request::new(CreateDsAlertGroupRequest {
        group_name: Some(group_name),
        alert_instance_ids: Some(alert_plugin_instance_ids),
        description,
        create_user_id: Some(user_id),
    });
    let res = client
        .clone()
        .create_ds_alert_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create alet groups error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn verify_alert_instance(group_name: &str) -> Result<()> {
    let client = _ds_alert_group_service_client().await?;
    let request = tonic::Request::new(VerifyAlertGroupRequest {
        group_name: group_name.to_string(),
    });
    client.clone().verify_alert_group(request).await?;
    Ok(())
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsAlertGroupsResponse> {
    info!(
        "list alet groups page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_alert_group_service_client().await?;
    let request = tonic::Request::new(ListDsAlertGroupsRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_alert_groups(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list alet groups error: {:?}", err);
            err
        })?;
    Ok(res)
}
pub async fn update(
    id: i32,
    group_name: String,
    alert_plugin_instance_ids: String,
    description: Option<String>,
    user_id: i32,
) -> Result<()> {
    let client = _ds_alert_group_service_client().await?;
    let request = tonic::Request::new(UpdateDsAlertGroupRequest {
        id,
        group_name: Some(group_name),
        alert_instance_ids: Some(alert_plugin_instance_ids),
        description,
        create_user_id: Some(user_id),
    });
    client
        .clone()
        .update_ds_alert_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update alet groups error: {:?}", err);
            err
        })?;
    Ok(())
}
