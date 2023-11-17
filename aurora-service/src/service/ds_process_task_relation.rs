use super::dao_service::AuroraRpcServer;
use proto::ds_process_task_relation::ds_process_task_relation_service_server::DsProcessTaskRelationService;

#[tonic::async_trait]
impl DsProcessTaskRelationService for AuroraRpcServer {
    async fn list_ds_process_task_relations(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::ListDsProcessTaskRelationsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation::ListDsProcessTaskRelationsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_task_relation(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::GetDsProcessTaskRelationRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelation>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_process_task_relation(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::CreateDsProcessTaskRelationRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelation>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_process_task_relation(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::UpdateDsProcessTaskRelationRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelation>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_process_task_relation(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::DeleteDsProcessTaskRelationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
