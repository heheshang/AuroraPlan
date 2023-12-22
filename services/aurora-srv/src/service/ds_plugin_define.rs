use super::dao_service::AuroraRpcServer;
use lib_common::core_error::error::{AuroraData, Error};
use lib_models::t_ds_plugin_define::Model;
use lib_proto::ds_plugin_define::{
    ds_plugin_define_service_server::DsPluginDefineService, GetDsPluginDefineByTypeResponse,
};

#[tonic::async_trait]
impl DsPluginDefineService for AuroraRpcServer {
    async fn list_ds_plugin_defines(
        &self,
        _req: tonic::Request<lib_proto::ds_plugin_define::ListDsPluginDefinesRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_plugin_define::ListDsPluginDefinesResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_plugin_define(
        &self,
        _req: tonic::Request<lib_proto::ds_plugin_define::GetDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_plugin_define::DsPluginDefine>, tonic::Status> {
        let id = _req.into_inner().id;
        let pool = &self.pool;
        let res = Model::query_by_id(id, pool).await.map_err(|_e| {
            tonic::Status::from_error(Box::new(Error::InternalServerErrorArgs(AuroraData::Null, None)))
        })?;
        Ok(tonic::Response::new(res.into()))
    }

    async fn create_ds_plugin_define(
        &self,
        _req: tonic::Request<lib_proto::ds_plugin_define::CreateDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_plugin_define::DsPluginDefine>, tonic::Status> {
        todo!()
    }

    async fn update_ds_plugin_define(
        &self,
        _req: tonic::Request<lib_proto::ds_plugin_define::UpdateDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_plugin_define::DsPluginDefine>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_plugin_define(
        &self,
        _req: tonic::Request<lib_proto::ds_plugin_define::DeleteDsPluginDefineRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_ds_plugin_define_by_type(
        &self,
        request: tonic::Request<lib_proto::ds_plugin_define::GetDsPluginDefineByTypeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_plugin_define::GetDsPluginDefineByTypeResponse>, tonic::Status>
    {
        let pool = &self.pool;
        let ui_type = request.into_inner().ui_type.to_lowercase();
        Model::query_by_type(&ui_type, pool)
            .await
            .map(|res| {
                tonic::Response::new(GetDsPluginDefineByTypeResponse {
                    ds_plugin_defines: res
                        .into_iter()
                        .map(|item| item.into())
                        .collect::<Vec<lib_proto::ds_plugin_define::DsPluginDefine>>(),
                })
            })
            .map_err(|_e| tonic::Status::from_error(Box::new(Error::InternalServerErrorArgs(AuroraData::Null, None))))
    }
}
