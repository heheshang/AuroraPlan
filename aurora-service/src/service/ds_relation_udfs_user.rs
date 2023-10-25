use super::dao_service::AuroraRpcServer;
use proto::ds_relation_udfs_user::ds_relation_udfs_user_service_server::DsRelationUdfsUserService;

#[tonic::async_trait]
impl DsRelationUdfsUserService for AuroraRpcServer {
    async fn list_ds_relation_udfs_users(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::ListDsRelationUdfsUsersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::ListDsRelationUdfsUsersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_udfs_user(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::GetDsRelationUdfsUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_udfs_user(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::CreateDsRelationUdfsUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_udfs_user(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::UpdateDsRelationUdfsUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_udfs_user(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::DeleteDsRelationUdfsUserRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
