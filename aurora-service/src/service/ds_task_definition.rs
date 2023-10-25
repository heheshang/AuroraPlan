use super::dao_service::AuroraRpcServer;
use proto::ds_task_definition::ds_task_definition_service_server::DsTaskDefinitionService;

#[tonic::async_trait]
impl DsTaskDefinitionService for AuroraRpcServer {
    async fn list_ds_task_definitions(
        &self,
        _req: tonic::Request<proto::ds_task_definition::ListDsTaskDefinitionsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::ListDsTaskDefinitionsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_definition(
        &self,
        _req: tonic::Request<proto::ds_task_definition::GetDsTaskDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_task_definition(
        &self,
        _req: tonic::Request<proto::ds_task_definition::CreateDsTaskDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_task_definition(
        &self,
        _req: tonic::Request<proto::ds_task_definition::UpdateDsTaskDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_task_definition(
        &self,
        _req: tonic::Request<proto::ds_task_definition::DeleteDsTaskDefinitionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
