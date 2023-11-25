use super::dao_service::AuroraRpcServer;

use entity::t_ds_environment_worker_group_relation::{Column, Entity};
use proto::ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_server::DsEnvironmentWorkerGroupRelationService;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub struct DsEnvironmentWorkerGroupServiceServer(pub AuroraRpcServer);
impl DsEnvironmentWorkerGroupServiceServer {
    pub async fn find_worker_groups_code(&self, code: i64) -> Vec<String> {
        let db = &self.0.db;
        Entity::find()
            .filter(Column::EnvironmentCode.eq(code))
            .all(db)
            .await
            .map(|v| v.into_iter().map(|m| m.worker_group).collect::<Vec<String>>())
            .unwrap_or_default()
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
