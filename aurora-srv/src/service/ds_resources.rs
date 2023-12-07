use super::dao_service::AuroraRpcServer;
use proto::ds_resources::ds_resource_service_server::DsResourceService;

#[tonic::async_trait]
impl DsResourceService for AuroraRpcServer {
    async fn list_ds_resources(
        &self,
        _req: tonic::Request<proto::ds_resources::ListDsResourcesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::ListDsResourcesResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_resource(
        &self,
        _req: tonic::Request<proto::ds_resources::GetDsResourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResource>, tonic::Status> {
        todo!()
    }

    async fn create_ds_resource(
        &self,
        _req: tonic::Request<proto::ds_resources::CreateDsResourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResource>, tonic::Status> {
        todo!()
    }

    async fn update_ds_resource(
        &self,
        _req: tonic::Request<proto::ds_resources::UpdateDsResourceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResource>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_resource(
        &self,
        _req: tonic::Request<proto::ds_resources::DeleteDsResourceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
