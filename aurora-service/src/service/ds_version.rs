use super::dao_service::AuroraRpcServer;
use proto::ds_version::ds_version_service_server::DsVersionService;

#[tonic::async_trait]
impl DsVersionService for AuroraRpcServer {
    async fn list_ds_versions(
        &self,
        _req: tonic::Request<proto::ds_version::ListDsVersionsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_version::ListDsVersionsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_version(
        &self,
        _req: tonic::Request<proto::ds_version::GetDsVersionRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersion>, tonic::Status> {
        todo!()
    }

    async fn create_ds_version(
        &self,
        _req: tonic::Request<proto::ds_version::CreateDsVersionRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersion>, tonic::Status> {
        todo!()
    }

    async fn update_ds_version(
        &self,
        _req: tonic::Request<proto::ds_version::UpdateDsVersionRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersion>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_version(
        &self,
        _req: tonic::Request<proto::ds_version::DeleteDsVersionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
