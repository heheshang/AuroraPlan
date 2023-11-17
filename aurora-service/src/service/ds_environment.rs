use super::dao_service::AuroraRpcServer;
use proto::ds_environment::ds_environment_service_server::DsEnvironmentService;

#[tonic::async_trait]
impl DsEnvironmentService for AuroraRpcServer {
    async fn list_ds_environments(
        &self,
        _req: tonic::Request<proto::ds_environment::ListDsEnvironmentsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::ListDsEnvironmentsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::GetDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn create_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::CreateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn update_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::UpdateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::DeleteDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
