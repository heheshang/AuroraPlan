use super::dao_service::AuroraRpcServer;
use proto::ds_process_definition_log::ds_process_definition_log_service_server::DsProcessDefinitionLogService;

#[tonic::async_trait]
impl DsProcessDefinitionLogService for AuroraRpcServer {
    async fn list_ds_process_definition_logs(
        &self,
        _req: tonic::Request<proto::ds_process_definition_log::ListDsProcessDefinitionLogsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition_log::ListDsProcessDefinitionLogsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_definition_log(
        &self,
        _req: tonic::Request<proto::ds_process_definition_log::GetDsProcessDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLog>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_process_definition_log(
        &self,
        _req: tonic::Request<proto::ds_process_definition_log::CreateDsProcessDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLog>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_process_definition_log(
        &self,
        _req: tonic::Request<proto::ds_process_definition_log::UpdateDsProcessDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLog>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_process_definition_log(
        &self,
        _req: tonic::Request<proto::ds_process_definition_log::DeleteDsProcessDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
