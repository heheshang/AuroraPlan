use super::dao_service::AuroraRpcServer;
use lib_proto::ds_k8s::ds_k8s_service_server::DsK8sService;

#[tonic::async_trait]
impl DsK8sService for AuroraRpcServer {
    async fn list_ds_k8ss(
        &self,
        _req: tonic::Request<lib_proto::ds_k8s::ListDsK8ssRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_k8s::ListDsK8ssResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_k8s(
        &self,
        _req: tonic::Request<lib_proto::ds_k8s::GetDsK8sRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_k8s::DsK8s>, tonic::Status> {
        todo!()
    }

    async fn create_ds_k8s(
        &self,
        _req: tonic::Request<lib_proto::ds_k8s::CreateDsK8sRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_k8s::DsK8s>, tonic::Status> {
        todo!()
    }

    async fn update_ds_k8s(
        &self,
        _req: tonic::Request<lib_proto::ds_k8s::UpdateDsK8sRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_k8s::DsK8s>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_k8s(
        &self,
        _req: tonic::Request<lib_proto::ds_k8s::DeleteDsK8sRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
