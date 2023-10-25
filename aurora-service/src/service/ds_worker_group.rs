use super::dao_service::AuroraRpcServer;
use proto::ds_worker_group::ds_worker_group_service_server::DsWorkerGroupService;

#[tonic::async_trait]
impl DsWorkerGroupService for AuroraRpcServer {
    async fn list_ds_worker_groups(
        &self,
        _req: tonic::Request<proto::ds_worker_group::ListDsWorkerGroupsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_worker_group::ListDsWorkerGroupsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::GetDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::CreateDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::UpdateDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_worker_group::DsWorkerGroup>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_worker_group(
        &self,
        _req: tonic::Request<proto::ds_worker_group::DeleteDsWorkerGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
