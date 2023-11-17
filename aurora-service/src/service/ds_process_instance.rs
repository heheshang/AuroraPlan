use super::dao_service::AuroraRpcServer;
use proto::ds_process_instance::ds_process_instance_service_server::DsProcessInstanceService;

#[tonic::async_trait]
impl DsProcessInstanceService for AuroraRpcServer {
    async fn list_ds_process_instances(
        &self,
        _req: tonic::Request<proto::ds_process_instance::ListDsProcessInstancesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_instance::ListDsProcessInstancesResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_process_instance(
        &self,
        _req: tonic::Request<proto::ds_process_instance::GetDsProcessInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_instance::DsProcessInstance>, tonic::Status> {
        todo!()
    }

    async fn create_ds_process_instance(
        &self,
        _req: tonic::Request<proto::ds_process_instance::CreateDsProcessInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_instance::DsProcessInstance>, tonic::Status> {
        todo!()
    }

    async fn update_ds_process_instance(
        &self,
        _req: tonic::Request<proto::ds_process_instance::UpdateDsProcessInstanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_instance::DsProcessInstance>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_process_instance(
        &self,
        _req: tonic::Request<proto::ds_process_instance::DeleteDsProcessInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
