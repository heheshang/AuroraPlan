use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use entity::{
    t_ds_queue::ActiveModel,
    t_ds_queue::{Column, Entity},
};
use proto::ds_queue::{ds_queue_service_server::DsQueueService, DsQueue, ListDsQueuesResponse};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde_json::json;
use tracing::{error, info};
type Result<T> = std::result::Result<T, tonic::Status>;

impl AuroraRpcServer {
    async fn queue_name_count(&self, queue_name: &str) -> Result<u64> {
        let conn = &self.conn;
        let res = Entity::find()
            .filter(Column::QueueName.eq(queue_name))
            .count(conn)
            .await
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;

        Ok(res)
    }
    async fn queue_value_count(&self, queue: &str) -> Result<u64> {
        let conn = &self.conn;
        let res = Entity::find()
            .filter(Column::Queue.eq(queue))
            .count(conn)
            .await
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;
        info!("res: {:?}", res);
        Ok(res)
    }
    async fn queue_name_count_extra(&self, queue_name: &str, id: i32) -> Result<u64> {
        let conn = &self.conn;
        let res = Entity::find()
            .filter(Column::QueueName.eq(queue_name))
            .filter(Column::Id.ne(id))
            .count(conn)
            .await
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;

        Ok(res)
    }
    async fn queue_value_count_extra(&self, queue: &str, id: i32) -> Result<u64> {
        let conn = &self.conn;
        let res = Entity::find()
            .filter(Column::Queue.eq(queue))
            .filter(Column::Id.ne(id))
            .count(conn)
            .await
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;
        info!("res: {:?}", res);
        Ok(res)
    }
    async fn find_by_id(&self, id: i32) -> Result<DsQueue> {
        let conn = &self.conn;
        let res: DsQueue = Entity::find_by_id(id)
            .one(conn)
            .await
            .map(|v| v.unwrap_or_default().into())
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;
        info!("res: {:?}", res);
        Ok(res)
    }
}

#[tonic::async_trait]
impl DsQueueService for AuroraRpcServer {
    async fn list_ds_queues(
        &self,
        _req: tonic::Request<proto::ds_queue::ListDsQueuesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::ListDsQueuesResponse>, tonic::Status>
    {
        info!("request: {:?}", _req);
        let conn = &self.conn;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{} ",
            search_val, page_size, page_num
        );
        let pages = Entity::find()
            .order_by_desc(Column::CreateTime)
            .filter(Column::Queue.like(format!("%{}%", search_val)))
            .paginate(conn, page_size);
        info!("query sql:{:#?}", pages);
        let page_num = match page_num {
            0 => 0,
            _ => page_num - 1,
        };
        let items = match pages.fetch_page(page_num).await {
            Ok(items) => items,
            Err(_) => {
                error!("fetch_page ds_queue error");
                vec![]
            }
        };
        let current_page = pages.cur_page();
        info!("current_page: {}", current_page);
        let (total, total_page) = match pages.num_items_and_pages().await {
            Ok(v) => (v.number_of_items, v.number_of_pages),
            Err(_) => {
                error!("num_items_and_pages ds_project_parameter error");
                (0, 0)
            }
        };
        info!("total: {}, total_page: {}", total, total_page);
        let start = (page_num) * page_size;
        info!("start: {}", start);
        let res = ListDsQueuesResponse {
            total,
            page_size,
            total_list: items.into_iter().map(|v| v.into()).collect(),
            current_page: current_page + 1,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

    async fn get_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::GetDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let conn = &self.conn;
        let queue = _req.get_ref().clone().queue;
        Entity::find()
            .filter(entity::t_ds_queue::Column::Queue.eq(queue))
            .one(conn)
            .await
            .map(|v| tonic::Response::new(v.unwrap_or_default().into()))
            .map_err(|_e| tonic::Status::from_error(Error::QueueNameExist(AuroraData::Null).into()))
    }

    async fn create_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::CreateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let conn = &self.conn;
        let queue = _req.get_ref().clone().queue;
        let queue_name = _req.get_ref().clone().queue_name;

        if Self::queue_name_count(self, &queue_name).await? >= 1 {
            return Err(tonic::Status::from_error(
                Error::QueueNameExist(AuroraData::Null).into(),
            ));
        }
        if Self::queue_value_count(self, &queue).await? >= 1 {
            return Err(tonic::Status::from_error(
                Error::QueueValueExist(AuroraData::Null).into(),
            ));
        }
        let res = ActiveModel {
            id: NotSet,
            queue: Set(Some(queue)),
            queue_name: Set(Some(queue_name)),
            create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
            update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        }
        .insert(conn)
        .await
        .map(|v| v.into())
        .map_err(|_e| {
            tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
        })?;
        Ok(tonic::Response::new(res))
    }

    async fn update_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::UpdateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        let conn = &self.conn;
        let queue = _req.get_ref().clone().ds_queue.unwrap_or_default().queue;
        let id = _req.get_ref().clone().ds_queue.unwrap_or_default().id;
        let queue_name = _req
            .get_ref()
            .clone()
            .ds_queue
            .unwrap_or_default()
            .queue_name;
        let entity = Self::find_by_id(self, id).await?;
        if Self::queue_name_count_extra(self, &queue_name.clone().unwrap_or_default(), id).await?
            >= 1
        {
            let err: AuroraErrorInfo = Error::QueueNameExist(json!(entity)).into();
            return Err(tonic::Status::from_error(Box::new(err)));
        }
        if Self::queue_value_count_extra(self, &queue.clone().unwrap_or_default(), id).await? >= 1 {
            let mut err: AuroraErrorInfo = Error::QueueValueExist(AuroraData::Null).into();
            err.error_data = json!(entity);
            return Err(tonic::Status::from_error(Box::new(err)));
        }

        ActiveModel {
            id: Set(id),
            queue: Set(queue),
            queue_name: Set(queue_name),
            create_time: NotSet,
            update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        }
        .update(conn)
        .await
        .map(|v| tonic::Response::new(v.into()))
        .map_err(|_e| {
            tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
        })
    }

    async fn delete_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::DeleteDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let conn = &self.conn;
        let res = Entity::delete_by_id(_req.get_ref().clone().id)
            .exec(conn)
            .await
            .map_err(|_e| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null).into())
            })?;

        if res.rows_affected > 0 {
            Ok(tonic::Response::new(()))
        } else {
            Err(tonic::Status::from_error(
                Error::InternalServerErrorArgs(AuroraData::Null).into(),
            ))
        }
    }

    async fn verify_ds_queue(
        &self,
        request: tonic::Request<proto::ds_queue::VerifyQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::VerifyQueue>, tonic::Status> {
        let queue = request.get_ref().clone().queue;
        let queue_name = request.get_ref().clone().queue_name;
        if Self::queue_name_count(self, &queue_name).await? >= 1 {
            return Err(tonic::Status::from_error(
                Error::QueueNameExist(AuroraData::Null).into(),
            ));
        }
        if Self::queue_value_count(self, &queue).await? >= 1 {
            return Err(tonic::Status::from_error(
                Error::QueueValueExist(AuroraData::Null).into(),
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
}
