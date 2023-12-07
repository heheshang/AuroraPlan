#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, Error};
// use entity::{
//     t_ds_worker_group::ActiveModel,
//     t_ds_worker_group::{Column, Entity},
// };
use log::{error, info};
use proto::ds_worker_group::ds_worker_group_service_server::DsWorkerGroupService;
// use sea_orm::Order::Desc;
// use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
#[tonic::async_trait]
impl DsWorkerGroupService for AuroraRpcServer {
    async fn list_ds_worker_groups(
        &self,
        _req: tonic::Request<proto::ds_worker_group::ListDsWorkerGroupsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::ListDsWorkerGroupsResponse>, tonic::Status> {
        // let db = &self.db;
        // let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        // let page_size = _req.get_ref().clone().page_size;
        // let page_num = _req.get_ref().clone().page_num;
        // info!(
        //     "search_val: {},page_size: {} ,page_num:{}",
        //     search_val, page_size, page_num
        // );
        // let pages = Entity::find()
        //     .order_by_desc(Column::CreateTime)
        //     .filter(Column::Name.like(format!("%{}%", search_val)))
        //     .paginate(db, page_size);
        // // info!("query sql:{:#?}", pages);
        // let page_num = match page_num {
        //     0 => 0,
        //     _ => page_num - 1,
        // };
        // let items = match pages.fetch_page(page_num).await {
        //     Ok(items) => items,
        //     Err(_) => {
        //         error!("fetch_page ds_worker_group error");
        //         vec![]
        //     }
        // };
        // let current_page = pages.cur_page();
        // info!("current_page: {}", current_page);
        // let (total, total_page) = match pages.num_items_and_pages().await {
        //     Ok(v) => (v.number_of_items, v.number_of_pages),
        //     Err(_) => {
        //         error!("num_items_and_pages ds_worker_group error");
        //         (0, 0)
        //     }
        // };
        // info!("total: {}, total_page: {}", total, total_page);
        // let start = (page_num) * page_size;
        // info!("start: {}", start);
        // let res = proto::ds_worker_group::ListDsWorkerGroupsResponse {
        //     total,
        //     page_size,
        //     total_list: items.into_iter().map(|v| v.into()).collect(),
        //     current_page: current_page + 1,
        //     start,
        //     total_page,
        // };
        // Ok(tonic::Response::new(res))
        todo!()
    }

    async fn get_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::GetDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status> {
        todo!()
    }

    async fn create_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::CreateDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status> {
        let db = &self.pool;
        // let name = _req.get_ref().name.clone();
        // let addr_list = _req.get_ref().addr_list.clone();
        // let res = ActiveModel {
        //     name: Set(name),
        //     addr_list: Set(addr_list),
        //     create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        //     update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        //     ..Default::default()
        // }
        // .insert(db)
        // .await
        // .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        // Ok(tonic::Response::new(res.into()))
        todo!()
    }

    async fn update_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::UpdateDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status> {
        let db = &self.pool;
        // let res = ActiveModel {
        //     id: sea_orm::ActiveValue::Set(_req.get_ref().id),
        //     name: sea_orm::ActiveValue::Set(_req.get_ref().name.clone()),
        //     update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        //     ..Default::default()
        // }
        // .update(db)
        // .await
        // .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        // Ok(tonic::Response::new(res.into()))
        todo!()
    }

    async fn delete_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::DeleteDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        // let db = &self.db;
        // Entity::delete_by_id(_req.get_ref().id)
        //     .exec(db)
        //     .await
        //     .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        // Ok(tonic::Response::new(()))
        todo!()
    }

    async fn all_ds_worker_groups(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::AllDsWorkerGroupsResponse>, tonic::Status> {
        // let db = &self.db;
        // Entity::find()
        //     .order_by(Column::CreateTime, Desc)
        //     .all(db)
        //     .await
        //     .map(|res| {
        //         tonic::Response::new(proto::ds_worker_group::AllDsWorkerGroupsResponse {
        //             data: res.into_iter().map(|v| v.name).collect(),
        //         })
        //     })
        //     .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
        todo!()
    }
}
