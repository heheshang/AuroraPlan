use super::dao_service::AuroraRpcServer;
use proto::ds_command::ds_command_service_server::DsCommandService;

#[tonic::async_trait]
impl DsCommandService for AuroraRpcServer {
    async fn list_ds_commands(
        &self,
        _req: tonic::Request<proto::ds_command::ListDsCommandsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::ListDsCommandsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_command(
        &self,
        _req: tonic::Request<proto::ds_command::GetDsCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommand>, tonic::Status> {
        todo!()
    }

    async fn create_ds_command(
        &self,
        _req: tonic::Request<proto::ds_command::CreateDsCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommand>, tonic::Status> {
        todo!()
    }

    async fn update_ds_command(
        &self,
        _req: tonic::Request<proto::ds_command::UpdateDsCommandRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommand>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_command(
        &self,
        _req: tonic::Request<proto::ds_command::DeleteDsCommandRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
