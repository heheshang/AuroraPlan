use super::dao_service::AuroraRpcServer;
use proto::ds_task_instance::ds_task_instance_service_server::DsTaskInstanceService;

#[tonic::async_trait]
impl DsTaskInstanceService for AuroraRpcServer {
    async fn list_ds_task_instances(
        &self,
        _req: tonic::Request<proto::ds_task_instance::ListDsTaskInstancesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_instance::ListDsTaskInstancesResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_task_instance(
        &self,
        _req: tonic::Request<proto::ds_task_instance::GetDsTaskInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_instance::DsTaskInstance>, tonic::Status> {
        todo!()
    }

    async fn create_ds_task_instance(
        &self,
        _req: tonic::Request<proto::ds_task_instance::CreateDsTaskInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_instance::DsTaskInstance>, tonic::Status> {
        todo!()
    }

    async fn update_ds_task_instance(
        &self,
        _req: tonic::Request<proto::ds_task_instance::UpdateDsTaskInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_instance::DsTaskInstance>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_task_instance(
        &self,
        _req: tonic::Request<proto::ds_task_instance::DeleteDsTaskInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
