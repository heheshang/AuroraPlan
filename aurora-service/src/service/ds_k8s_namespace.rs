use super::dao_service::AuroraRpcServer;
use proto::ds_k8s_namespace::ds_k8s_namespace_service_server::DsK8sNamespaceService;
#[tonic::async_trait]
impl DsK8sNamespaceService for AuroraRpcServer {
    async fn list_ds_k8s_namespaces(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::ListDsK8sNamespacesRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_k8s_namespace::ListDsK8sNamespacesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_k8s_namespace(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::GetDsK8sNamespaceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s_namespace::DsK8sNamespace>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_k8s_namespace(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::CreateDsK8sNamespaceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s_namespace::DsK8sNamespace>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_k8s_namespace(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::UpdateDsK8sNamespaceRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s_namespace::DsK8sNamespace>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_k8s_namespace(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::DeleteDsK8sNamespaceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
