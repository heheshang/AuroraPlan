use super::dao_service::AuroraRpcServer;
use proto::ds_relation_resources_user::ds_relation_resources_user_service_server::DsRelationResourcesUserService;

#[tonic::async_trait]
impl DsRelationResourcesUserService for AuroraRpcServer {
    async fn list_ds_relation_resources_users(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::ListDsRelationResourcesUsersRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::ListDsRelationResourcesUsersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_resources_user(
        &self,
        _req: tonic::Request<proto::ds_relation_resources_user::GetDsRelationResourcesUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_resources_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::CreateDsRelationResourcesUserRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_resources_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::UpdateDsRelationResourcesUserRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_resources_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::DeleteDsRelationResourcesUserRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
