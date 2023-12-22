use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_alert_plugin_instance::Model;
use lib_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use lib_proto::ds_alert_plugin_instance::ds_alert_plugin_instance_service_server::DsAlertPluginInstanceService;
use tracing::{error, info};

#[tonic::async_trait]
impl DsAlertPluginInstanceService for AuroraRpcServer {
    async fn list_ds_alert_plugin_instances(
        &self,
        _request: tonic::Request<lib_proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesResponse>,
        tonic::Status,
    > {
        let pool = &self.pool;
        let page_size = _request.get_ref().page_size;
        let page_num = _request.get_ref().page_num;
        let search_val = _request.get_ref().search_val.clone().unwrap_or_default();
        Model::page(&search_val, page_num, page_size, pool)
            .await
            .map(|(items, total_page, total, start, current_page)| {
                tonic::Response::new(
                    lib_proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesResponse {
                        total_list: items.into_iter().map(|instance| instance.into()).collect(),
                        total_page,
                        total,
                        start,
                        current_page,
                        page_size,
                    },
                )
            })
            .map_err(|_e| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn get_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_alert_plugin_instance::GetDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert_plugin_instance::DsAlertPluginInstance>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_alert_plugin_instance::CreateDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert_plugin_instance::DsAlertPluginInstance>, tonic::Status>
    {
        let pool = &self.pool;
        let instance_name = &_req.get_ref().instance_name;
        let plugin_define_id = _req.get_ref().plugin_define_id;
        let plugin_instance_params = &_req.get_ref().plugin_instance_params;
        Model::_create(
            plugin_define_id,
            plugin_instance_params.clone(),
            instance_name.clone(),
            pool,
        )
        .await
        .map(|instance| tonic::Response::new(instance.into()))
        .map_err(|_e| {
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })
    }

    async fn update_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_alert_plugin_instance::UpdateDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert_plugin_instance::DsAlertPluginInstance>, tonic::Status>
    {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        let instance_name = &_req.get_ref().instance_name;
        let plugin_instance_params = &_req.get_ref().plugin_instance_params;
        Model::_update(id, plugin_instance_params.clone(), instance_name.clone(), pool)
            .await
            .map(|instance| tonic::Response::new(instance.into()))
            .map_err(|_e| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn delete_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_alert_plugin_instance::DeleteDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let id = _req.get_ref().id;
        let pool = &self.pool;
        Model::_delete(id, pool).await.map(|_| tonic::Response::new(())).map_err(|_e| {
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })
    }

    async fn verify_alert_plugin_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_alert_plugin_instance::VerifyAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let instance_name = _req.into_inner().instance_name;
        info!("verify_alert_plugin_instance: {}", instance_name);
        let instance = Model::find_by_instance_name(instance_name, pool).await.map_err(|_e| {
            error!("verify_alert_plugin_instance error: {:?}", _e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })?;
        match instance {
            Some(_) => {
                let err: AuroraErrorInfo = Error::PluginInstanceAlreadyExit(AuroraData::Null, None).into();
                Err(tonic::Status::from_error(Box::new(err)))
            }
            None => Ok(tonic::Response::new(())),
        }
    }

    async fn all_alert_plugin_instance(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_alert_plugin_instance::AllDsAlertPluginInstancesResponse>,
        tonic::Status,
    > {
        let pool = &self.pool;
        let instances = Model::all(pool).await.map_err(|_e| {
            error!("all_alert_plugin_instance error: {:?}", _e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })?;
        Ok(tonic::Response::new(
            lib_proto::ds_alert_plugin_instance::AllDsAlertPluginInstancesResponse {
                total_list: instances.into_iter().map(|instance| instance.into()).collect(),
            },
        ))
    }
}
