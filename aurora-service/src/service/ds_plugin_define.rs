use super::dao_service::AuroraRpcServer;
use proto::ds_plugin_define::ds_plugin_define_service_server::DsPluginDefineService;

#[tonic::async_trait]
impl DsPluginDefineService for AuroraRpcServer {
    async fn list_ds_plugin_defines(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::ListDsPluginDefinesRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_plugin_define::ListDsPluginDefinesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_plugin_define(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::GetDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_plugin_define::DsPluginDefine>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_plugin_define(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::CreateDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_plugin_define::DsPluginDefine>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_plugin_define(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::UpdateDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_plugin_define::DsPluginDefine>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_plugin_define(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::DeleteDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
