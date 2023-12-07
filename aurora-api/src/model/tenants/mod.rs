use crate::{model::client::service::_ds_tenant_service_client, web::bean::response::tenants::Tenant};
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_tenant::{
    CreateDsTenantRequest, DeleteDsTenantRequest, ListDsTenantsRequest, ListDsTenantsResponse, UpdateDsTenantRequest,
    VerifyDsTenantRequest,
};
use log::{error, info};

pub async fn delete(id: i32) -> Result<()> {
    let client = _ds_tenant_service_client().await?;
    let request = tonic::Request::new(DeleteDsTenantRequest { id });
    client
        .clone()
        .delete_ds_tenant(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete tenants error: {:?}", err);
            err
        })?;
    Ok(())
}

pub async fn create(tenant_code: String, description: Option<String>, queue_id: Option<i32>) -> Result<Tenant> {
    let client = _ds_tenant_service_client().await?;
    let request = tonic::Request::new(CreateDsTenantRequest {
        tenant_code,
        description,
        queue_id,
    });
    let res = client
        .clone()
        .create_ds_tenant(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create tenants error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn verify_code(tenant_code: &str) -> Result<()> {
    let client = _ds_tenant_service_client().await?;
    let request = tonic::Request::new(VerifyDsTenantRequest {
        tenant_code: tenant_code.to_string(),
    });
    client.clone().verify_ds_tenant(request).await?;
    Ok(())
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsTenantsResponse> {
    info!(
        "list tenants page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_tenant_service_client().await?;
    let request = tonic::Request::new(ListDsTenantsRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_tenants(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list tenants error: {:?}", err);
            err
        })?;
    Ok(res)
}

pub async fn update(id: i32, description: &str, queue_id: &i32) -> Result<()> {
    let client = _ds_tenant_service_client().await?;
    let request = tonic::Request::new(UpdateDsTenantRequest {
        id,
        description: description.to_string(),
        queue_id: *queue_id,
    });
    client
        .clone()
        .update_ds_tenant(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update tenants error: {:?}", err);
            err
        })?;
    Ok(())
}
