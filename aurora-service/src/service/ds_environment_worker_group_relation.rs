use super::dao_service::AuroraRpcServer;
use proto::ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_server::DsEnvironmentWorkerGroupRelationService;

#[tonic::async_trait]
impl DsEnvironmentWorkerGroupRelationService for AuroraRpcServer {
    async fn list_ds_environment_worker_group_relations(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::ListDsEnvironmentWorkerGroupRelationsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment_worker_group_relation::ListDsEnvironmentWorkerGroupRelationsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_environment_worker_group_relation(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::GetDsEnvironmentWorkerGroupRelationRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelation>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_environment_worker_group_relation(
        &self,
        _req: tonic::Request<
            proto::ds_environment_worker_group_relation::CreateDsEnvironmentWorkerGroupRelationRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelation>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_environment_worker_group_relation(
        &self,
        _req: tonic::Request<
            proto::ds_environment_worker_group_relation::UpdateDsEnvironmentWorkerGroupRelationRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelation>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_environment_worker_group_relation(
        &self,
        _req: tonic::Request<
            proto::ds_environment_worker_group_relation::DeleteDsEnvironmentWorkerGroupRelationRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
