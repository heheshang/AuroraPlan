use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_queue::Model;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use log::info;
use proto::ds_queue::ds_queue_service_server::DsQueueService;

use serde_json::json;

#[tonic::async_trait]
impl DsQueueService for AuroraRpcServer {
    async fn list_ds_queues(
        &self,
        _req: tonic::Request<proto::ds_queue::ListDsQueuesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::ListDsQueuesResponse>, tonic::Status> {
        info!("request: {:?}", _req);
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{} ",
            search_val, page_size, page_num
        );
        Model::page(&search_val, page_num, page_size, pool)
            .await
            .map(|(items, total_page, total, start, cur_page)| {
                let res = proto::ds_queue::ListDsQueuesResponse {
                    total,
                    page_size,
                    total_list: items.into_iter().map(|v| v.into()).collect(),
                    current_page: cur_page,
                    start,
                    total_page,
                };
                tonic::Response::new(res)
            })
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn get_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::GetDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let pool = &self.pool;
        let queue = _req.get_ref().clone().queue;
        Model::find_by_queue(queue, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn create_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::CreateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let pool = &self.pool;
        let queue = _req.get_ref().clone().queue;
        let queue_name = _req.get_ref().clone().queue_name;

        if Model::queue_name_count(queue_name.clone(), pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            return Err(tonic::Status::from_error(
                Error::QueueNameExist(AuroraData::Null, Some(vec![queue_name.clone()])).into(),
            ));
        }
        if Model::queue_value_count(queue.clone(), pool)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            return Err(tonic::Status::from_error(
                Error::QueueValueExist(AuroraData::Null, Some(vec![queue.clone()])).into(),
            ));
        }
        Model::create(queue, queue_name, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn update_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::UpdateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let pool = &self.pool;
        let queue = _req.get_ref().clone().ds_queue.unwrap_or_default().queue;
        let id = _req.get_ref().clone().ds_queue.unwrap_or_default().id;
        let queue_name = _req.get_ref().clone().ds_queue.unwrap_or_default().queue_name;
        let entity = Model::find_by_id(id, pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        if Model::queue_name_count_extra(&queue_name.clone().unwrap_or_default(), id, pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            let err: AuroraErrorInfo =
                Error::QueueNameExist(json!(entity), Some(vec![queue_name.unwrap_or_default()])).into();
            return Err(tonic::Status::from_error(Box::new(err)));
        }
        if Model::queue_value_count_extra(&queue.clone().unwrap_or_default(), id, pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            let mut err: AuroraErrorInfo = Error::QueueValueExist(AuroraData::Null, None).into();
            err.error_data = json!(entity);
            return Err(tonic::Status::from_error(Box::new(err)));
        }
        Model::update(id, queue_name, queue, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn delete_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::DeleteDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let conn = &self.pool;

        Model::delete(_req.get_ref().clone().id, conn)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn verify_ds_queue(
        &self,
        request: tonic::Request<proto::ds_queue::VerifyQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::VerifyQueue>, tonic::Status> {
        let queue = request.get_ref().clone().queue;
        let pool = &self.pool;
        let queue_name = request.get_ref().clone().queue_name;
        if Model::queue_name_count(queue_name.clone(), pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            return Err(tonic::Status::from_error(
                Error::QueueNameExist(AuroraData::Null, Some(vec![queue_name.clone()])).into(),
            ));
        }
        if Model::queue_value_count(queue.clone(), pool)
            .await
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?
            >= 1
        {
            return Err(tonic::Status::from_error(
                Error::QueueValueExist(AuroraData::Null, Some(vec![queue.clone()])).into(),
            ));
        }

        Ok(tonic::Response::new(proto::ds_queue::VerifyQueue {
            ds_queue: Some(proto::ds_queue::DsQueue {
                id: 0,
                queue: Some(queue),
                queue_name: Some(queue_name),
                create_time: None,
                update_time: None,
            }),
        }))
    }

    async fn all_ds_queues(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::AllDsQueuesResponse>, tonic::Status> {
        let pool = &self.pool;
        Model::all(pool)
            .await
            .map(|v| {
                tonic::Response::new(proto::ds_queue::AllDsQueuesResponse {
                    data: v.into_iter().map(|v| v.into()).collect(),
                })
            })
            .map_err(|_e| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }
}
