use crate::{
    model::client::service::_ds_worker_group_service_client, web::bean::response::worker_groups::WorkerGroups,
};
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_worker_group::{
    CreateDsWorkerGroupRequest, DeleteDsWorkerGroupRequest, ListDsWorkerGroupsRequest, ListDsWorkerGroupsResponse,
    UpdateDsWorkerGroupRequest,
};
use log::{error, info};

pub async fn delete(id: i32) -> Result<()> {
    let client = _ds_worker_group_service_client().await?;
    let request = tonic::Request::new(DeleteDsWorkerGroupRequest { id });
    client
        .clone()
        .delete_ds_worker_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete worker_groups error: {:?}", err);
            err
        })?;
    Ok(())
}

pub async fn create(name: String, addr_list: String) -> Result<WorkerGroups> {
    let client = _ds_worker_group_service_client().await?;
    let request = tonic::Request::new(CreateDsWorkerGroupRequest {
        name,
        addr_list: Some(addr_list),
    });
    let res = client
        .clone()
        .create_ds_worker_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create worker_groups error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsWorkerGroupsResponse> {
    info!(
        "list worker_groups page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_worker_group_service_client().await?;
    let request = tonic::Request::new(ListDsWorkerGroupsRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_worker_groups(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list worker_groups error: {:?}", err);
            err
        })?;
    Ok(res)
}

pub async fn update(id: i64, name: &str, addr_list: &str) -> Result<()> {
    let client = _ds_worker_group_service_client().await?;
    let request = tonic::Request::new(UpdateDsWorkerGroupRequest {
        id,
        name: name.to_string(),
        addr_list: Some(addr_list.to_string()),
    });
    client
        .clone()
        .update_ds_worker_group(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update worker_groups error: {:?}", err);
            err
        })?;
    Ok(())
}
pub async fn all() -> Result<Vec<String>> {
    let client = _ds_worker_group_service_client().await?;
    let request = tonic::Request::new(());
    let res = client
        .clone()
        .all_ds_worker_groups(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list worker_groups error: {:?}", err);
            err
        })?;
    Ok(res.data)
}
