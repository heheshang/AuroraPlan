#![allow(dead_code)]

use crate::{
    model::client::service::_ds_queue_service_client, web::bean::response::queue::Queue,
    web::bean::response::queue::VerifyQueue,
};
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::ds_queue::{
    CreateDsQueueRequest, DeleteDsQueueRequest, ListDsQueuesRequest, ListDsQueuesResponse, UpdateDsQueueRequest,
    VerifyQueueRequest,
};
use tracing::{error, info};

pub async fn list_all_queue() -> Result<Vec<Queue>> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(());
    let res = client
        .clone()
        .all_ds_queues(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list queue error: {:?}", err);
            err
        })?;
    Ok(res.data.into_iter().map(|v| v.into()).collect())
}

pub async fn create(queue: &str, queue_name: &str) -> Result<Queue> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(CreateDsQueueRequest {
        queue: queue.to_string(),
        queue_name: queue_name.to_string(),
    });
    let res = client
        .clone()
        .create_ds_queue(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create queue error: {:?}", err);
            err
        })?;
    Ok(res.into())
}
pub async fn verify(queue: &str, queue_name: &str) -> Result<VerifyQueue> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(VerifyQueueRequest {
        queue: queue.to_string(),
        queue_name: queue_name.to_string(),
    });
    let res = client
        .clone()
        .verify_ds_queue(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("verify queue error: {:?}", err);
            err
        })?;
    info!("res: {:?}", res);
    Ok(res.ds_queue.unwrap().into())
}

pub async fn list(page_num: &i64, page_size: &i64, search_val: &Option<String>) -> Result<ListDsQueuesResponse> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(ListDsQueuesRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_queues(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list queue error: {:?}", err);
            err
        })?;
    Ok(res)
}
pub async fn delete(id: i32) -> Result<()> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(DeleteDsQueueRequest { id });
    client
        .clone()
        .delete_ds_queue(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete queue error: {:?}", err);
            err
        })
}

pub async fn update(id: i32, queue: &str, queue_name: &str) -> Result<Queue> {
    let client = _ds_queue_service_client().await?;
    let request = tonic::Request::new(UpdateDsQueueRequest {
        ds_queue: Some(aurora_proto::ds_queue::DsQueue {
            id,
            queue: Some(queue.to_string()),
            queue_name: Some(queue_name.to_string()),
            create_time: None,
            update_time: None,
        }),
    });
    Ok(client
        .clone()
        .update_ds_queue(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            error!("update queue error: {:?}", e);
            Error::from(e)
        })?
        .into())
}
