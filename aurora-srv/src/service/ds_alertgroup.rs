use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_alertgroup::Model;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use proto::ds_alertgroup::ds_alert_group_service_server::DsAlertGroupService;
use tracing::error;

#[tonic::async_trait]
impl DsAlertGroupService for AuroraRpcServer {
    async fn list_ds_alert_groups(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::ListDsAlertGroupsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::ListDsAlertGroupsResponse>, tonic::Status> {
        let pool = &self.pool;
        let page_size = _req.get_ref().page_size;
        let page_num = _req.get_ref().page_num;
        let search_val = _req.get_ref().search_val.clone().unwrap_or_default();
        Model::page(&search_val, page_num, page_size, pool)
            .await
            .map(|(items, total_page, total, start, current_page)| {
                tonic::Response::new(proto::ds_alertgroup::ListDsAlertGroupsResponse {
                    total_list: items.into_iter().map(|instance| instance.into()).collect(),
                    total_page,
                    total,
                    start,
                    current_page,
                    page_size,
                })
            })
            .map_err(|_e| {
                error!("list_ds_alert_groups error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn get_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::GetDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status> {
        todo!()
    }

    async fn create_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::CreateDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status> {
        let pool = &self.pool;
        let create_user_id = _req.get_ref().create_user_id;
        let group_name = _req.get_ref().group_name.clone();
        let description = _req.get_ref().description.clone();
        let alert_instance_ids = _req.get_ref().alert_instance_ids.clone();
        Model::_create(alert_instance_ids, create_user_id, group_name, description, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("create_ds_alert_group error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn update_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::UpdateDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        let create_user_id = _req.get_ref().create_user_id;
        let group_name = _req.get_ref().group_name.clone();
        let description = _req.get_ref().description.clone();
        let alert_instance_ids = _req.get_ref().alert_instance_ids.clone();
        Model::_update(id, alert_instance_ids, create_user_id, group_name, description, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("update_ds_alert_group error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn delete_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::DeleteDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        Model::_delete(id, pool).await.map(|_| tonic::Response::new(())).map_err(|_e| {
            error!("delete_ds_alert_group error: {:?}", _e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })
    }

    async fn verify_alert_group(
        &self,
        _request: tonic::Request<proto::ds_alertgroup::VerifyAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let group_name = _request.get_ref().group_name.clone();
        let res = Model::_find_by_name(group_name, pool).await.map_err(|e| {
            error!("verify_alert_group error: {:?}", e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })?;

        match res {
            None => Ok(tonic::Response::new(())),
            Some(_) => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::AlertGroupExist(AuroraData::Null, None).into(),
            ))),
        }
    }
}
