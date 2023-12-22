use crate::{
    model::client::service::_ds_alert_plugin_instance_service_client,
    web::bean::response::alert_plugin_instances::AlertPluginInstance,
};
use lib_common::{core_error::error::Error, core_results::results::Result};
use lib_proto::ds_alert_plugin_instance::{
    CreateDsAlertPluginInstanceRequest, DeleteDsAlertPluginInstanceRequest, ListDsAlertPluginInstancesRequest,
    ListDsAlertPluginInstancesResponse, UpdateDsAlertPluginInstanceRequest, VerifyAlertPluginInstanceRequest,
};
use tracing::{error, info};

pub async fn all() -> Result<Vec<AlertPluginInstance>> {
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(());
    let res = client
        .clone()
        .all_alert_plugin_instance(request)
        .await
        .map(|res| res.into_inner().total_list.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list tenants error: {:?}", err);
            err
        })?;
    Ok(res)
}
pub async fn delete(id: i32) -> Result<()> {
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(DeleteDsAlertPluginInstanceRequest { id });
    client
        .clone()
        .delete_ds_alert_plugin_instance(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete tenants error: {:?}", err);
            err
        })?;
    Ok(())
}

pub async fn create(
    instance_name: String,
    plugin_define_id: i32,
    plugin_instance_params: String,
) -> Result<AlertPluginInstance> {
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(CreateDsAlertPluginInstanceRequest {
        instance_name: Some(instance_name),
        plugin_instance_params: Some(plugin_instance_params),
        plugin_define_id,
    });
    let res = client
        .clone()
        .create_ds_alert_plugin_instance(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create tenants error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn verify_alert_instance(instance_name: &str) -> Result<()> {
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(VerifyAlertPluginInstanceRequest {
        instance_name: instance_name.to_string(),
    });
    client.clone().verify_alert_plugin_instance(request).await?;
    Ok(())
}
pub async fn list(
    page_num: &i64,
    page_size: &i64,
    search_val: &Option<String>,
) -> Result<ListDsAlertPluginInstancesResponse> {
    info!(
        "list tenants page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(ListDsAlertPluginInstancesRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_alert_plugin_instances(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list tenants error: {:?}", err);
            err
        })?;
    Ok(res)
}
pub async fn update(id: i32, instance_name: String, plugin_instance_params: String) -> Result<()> {
    let client = _ds_alert_plugin_instance_service_client().await?;
    let request = tonic::Request::new(UpdateDsAlertPluginInstanceRequest {
        id,
        instance_name: Some(instance_name),
        plugin_instance_params: Some(plugin_instance_params),
    });
    client
        .clone()
        .update_ds_alert_plugin_instance(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update tenants error: {:?}", err);
            err
        })?;
    Ok(())
}
