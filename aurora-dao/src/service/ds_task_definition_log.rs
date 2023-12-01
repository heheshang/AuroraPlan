use super::dao_service::AuroraRpcServer;
use proto::ds_task_definition_log::ds_task_definition_log_service_server::DsTaskDefinitionLogService;

#[tonic::async_trait]
impl DsTaskDefinitionLogService for AuroraRpcServer {
    async fn list_ds_task_definition_logs(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::ListDsTaskDefinitionLogsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition_log::ListDsTaskDefinitionLogsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_definition_log(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::GetDsTaskDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLog>, tonic::Status> {
        todo!()
    }

    async fn create_ds_task_definition_log(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::CreateDsTaskDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLog>, tonic::Status> {
        todo!()
    }

    async fn update_ds_task_definition_log(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::UpdateDsTaskDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLog>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_task_definition_log(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::DeleteDsTaskDefinitionLogRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
