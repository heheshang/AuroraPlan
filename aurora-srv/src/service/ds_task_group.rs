use super::dao_service::AuroraRpcServer;
use proto::ds_task_group::ds_task_group_service_server::DsTaskGroupService;

#[tonic::async_trait]
impl DsTaskGroupService for AuroraRpcServer {
    async fn list_ds_task_groups(
        &self,
        _req: tonic::Request<proto::ds_task_group::ListDsTaskGroupsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::ListDsTaskGroupsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_task_group(
        &self,
        _req: tonic::Request<proto::ds_task_group::GetDsTaskGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroup>, tonic::Status> {
        todo!()
    }

    async fn create_ds_task_group(
        &self,
        _req: tonic::Request<proto::ds_task_group::CreateDsTaskGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroup>, tonic::Status> {
        todo!()
    }

    async fn update_ds_task_group(
        &self,
        _req: tonic::Request<proto::ds_task_group::UpdateDsTaskGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroup>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_task_group(
        &self,
        _req: tonic::Request<proto::ds_task_group::DeleteDsTaskGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
