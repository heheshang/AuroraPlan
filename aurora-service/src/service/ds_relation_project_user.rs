use super::dao_service::AuroraRpcServer;
use proto::ds_relation_project_user::ds_relation_project_user_service_server::DsRelationProjectUserService;

#[tonic::async_trait]
impl DsRelationProjectUserService for AuroraRpcServer {
    async fn list_ds_relation_project_users(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::ListDsRelationProjectUsersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::ListDsRelationProjectUsersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_project_user(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::GetDsRelationProjectUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_project_user(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::CreateDsRelationProjectUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_project_user(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::UpdateDsRelationProjectUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_project_user(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::DeleteDsRelationProjectUserRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
