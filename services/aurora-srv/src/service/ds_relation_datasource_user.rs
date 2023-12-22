use super::dao_service::AuroraRpcServer;
use lib_proto::ds_relation_datasource_user::ds_relation_datasource_user_service_server::DsRelationDatasourceUserService;

#[tonic::async_trait]
impl DsRelationDatasourceUserService for AuroraRpcServer {
    async fn list_ds_relation_datasource_users(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_datasource_user::ListDsRelationDatasourceUsersRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_datasource_user::ListDsRelationDatasourceUsersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_datasource_user(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_datasource_user::GetDsRelationDatasourceUserRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_datasource_user::DsRelationDatasourceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_datasource_user(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_datasource_user::CreateDsRelationDatasourceUserRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_datasource_user::DsRelationDatasourceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_datasource_user(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_datasource_user::UpdateDsRelationDatasourceUserRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_datasource_user::DsRelationDatasourceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_datasource_user(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_datasource_user::DeleteDsRelationDatasourceUserRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
