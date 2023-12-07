use super::dao_service::AuroraRpcServer;

use proto::ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_server::DsEnvironmentWorkerGroupRelationService;

pub struct DsEnvironmentWorkerGroupServiceServer(pub AuroraRpcServer);
impl DsEnvironmentWorkerGroupServiceServer {
    #[allow(dead_code)]
    pub async fn find_worker_groups_code(&self, _code: i64) -> Vec<String> {
        let _pool = &self.0.pool;
        // Entity::find()
        //     .filter(Column::EnvironmentCode.eq(code))
        //     .all(db)
        //     .await
        //     .map(|v| v.into_iter().map(|m| m.worker_group).collect::<Vec<String>>())
        //     .unwrap_or_default()
        todo!()
    }
}

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
