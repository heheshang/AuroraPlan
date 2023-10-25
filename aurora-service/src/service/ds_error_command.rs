use super::dao_service::AuroraRpcServer;
use proto::ds_error_command::ds_error_command_service_server::DsErrorCommandService;

#[tonic::async_trait]
impl DsErrorCommandService for AuroraRpcServer {
    async fn list_ds_error_commands(
        &self,
        _req: tonic::Request<proto::ds_error_command::ListDsErrorCommandsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_error_command::ListDsErrorCommandsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_error_command(
        &self,
        _req: tonic::Request<proto::ds_error_command::GetDsErrorCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_error_command::DsErrorCommand>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_error_command(
        &self,
        _req: tonic::Request<proto::ds_error_command::CreateDsErrorCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_error_command::DsErrorCommand>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_error_command(
        &self,
        _req: tonic::Request<proto::ds_error_command::UpdateDsErrorCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_error_command::DsErrorCommand>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_error_command(
        &self,
        _req: tonic::Request<proto::ds_error_command::DeleteDsErrorCommandRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
