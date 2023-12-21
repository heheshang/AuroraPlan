use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, Error};

use crate::models::t_ds_worker_group::Model;
use proto::ds_worker_group::ds_worker_group_service_server::DsWorkerGroupService;
use tracing::info;
#[tonic::async_trait]
impl DsWorkerGroupService for AuroraRpcServer {
    async fn list_ds_worker_groups(
        &self,
        _req: tonic::Request<proto::ds_worker_group::ListDsWorkerGroupsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::ListDsWorkerGroupsResponse>, tonic::Status> {
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{}",
            search_val, page_size, page_num
        );
        Model::page(&search_val, page_num, page_size, pool)
            .await
            .map(|v| {
                tonic::Response::new(proto::ds_worker_group::ListDsWorkerGroupsResponse {
                    total: v.2,
                    page_size,
                    total_list: v.0.into_iter().map(|v| v.into()).collect(),
                    current_page: v.4,
                    start: v.3,
                    total_page: v.1,
                })
            })
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
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
        let pool = &self.pool;

        let name = _req.get_ref().name.clone();
        let addr_list = _req.get_ref().addr_list.clone();
        Model::create(name, addr_list, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn update_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::UpdateDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        let name = _req.get_ref().name.clone();
        Model::update(id, Some(name), pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn delete_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::DeleteDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        Model::delete(id.into(), pool)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn all_ds_worker_groups(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::AllDsWorkerGroupsResponse>, tonic::Status> {
        let pool = &self.pool;
        Model::all(pool)
            .await
            .map(|v| {
                tonic::Response::new(proto::ds_worker_group::AllDsWorkerGroupsResponse {
                    data: v.into_iter().map(|v| v.name).collect(),
                })
            })
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }
}
