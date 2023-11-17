use super::dao_service::AuroraRpcServer;

use proto::ds_datasource::ds_datasource_service_server::DsDatasourceService;

#[tonic::async_trait]
impl DsDatasourceService for AuroraRpcServer {
    async fn list_ds_datasources(
        &self,
        _req: tonic::Request<proto::ds_datasource::ListDsDatasourcesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::ListDsDatasourcesResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_datasource(
        &self,
        _req: tonic::Request<proto::ds_datasource::GetDsDatasourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasource>, tonic::Status> {
        todo!()
    }

    async fn create_ds_datasource(
        &self,
        _req: tonic::Request<proto::ds_datasource::CreateDsDatasourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasource>, tonic::Status> {
        todo!()
    }

    async fn update_ds_datasource(
        &self,
        _req: tonic::Request<proto::ds_datasource::UpdateDsDatasourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasource>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_datasource(
        &self,
        _req: tonic::Request<proto::ds_datasource::DeleteDsDatasourceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
