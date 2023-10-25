use super::dao_service::AuroraRpcServer;
use proto::ds_tenant::ds_tenant_service_server::DsTenantService;
#[tonic::async_trait]
impl DsTenantService for AuroraRpcServer {
    async fn list_ds_tenants(
        &self,
        _req: tonic::Request<proto::ds_tenant::ListDsTenantsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::ListDsTenantsResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::GetDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenant>, tonic::Status> {
        todo!()
    }

    async fn create_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::CreateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenant>, tonic::Status> {
        todo!()
    }

    async fn update_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::UpdateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenant>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::DeleteDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
