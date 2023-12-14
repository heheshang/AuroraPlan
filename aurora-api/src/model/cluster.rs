use crate::{model::client::service::_ds_cluster_service_client, web::bean::response::cluster::Cluster};
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_cluster::{
    CreateDsClusterRequest, DeleteDsClusterRequest, ListDsClustersRequest, ListDsClustersResponse,
    UpdateDsClusterRequest, VerifyClusterRequest,
};
use log::{error, info};

pub async fn delete(code: i64) -> Result<()> {
    let client = _ds_cluster_service_client().await?;
    let request = tonic::Request::new(DeleteDsClusterRequest { code });
    client
        .clone()
        .delete_ds_cluster(request)
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
    name: Option<String>,
    config: Option<String>,
    description: Option<String>,
    operator: Option<i32>,
) -> Result<Cluster> {
    let client = _ds_cluster_service_client().await?;
    let request = tonic::Request::new(CreateDsClusterRequest {
        name,
        config,
        description,
        operator,
    });
    let res = client
        .clone()
        .create_ds_cluster(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create alet groups error: {:?}", err);
            err
        })?;
    Ok(res.into())
}

pub async fn verify(name: &str) -> Result<()> {
    let client = _ds_cluster_service_client().await?;
    let request = tonic::Request::new(VerifyClusterRequest { name: name.to_string() });
    client.clone().verify_ds_cluster(request).await?;
    Ok(())
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsClustersResponse> {
    info!(
        "list alet groups page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_cluster_service_client().await?;
    let request = tonic::Request::new(ListDsClustersRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_clusters(request)
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
    code: i64,
    name: Option<String>,
    config: Option<String>,
    description: Option<String>,
    operator: Option<i32>,
) -> Result<()> {
    let client = _ds_cluster_service_client().await?;
    let request = tonic::Request::new(UpdateDsClusterRequest {
        code,
        name,
        config,
        description,
        operator,
    });
    client
        .clone()
        .update_ds_cluster(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("update alet groups error: {:?}", err);
            err
        })?;
    Ok(())
}
