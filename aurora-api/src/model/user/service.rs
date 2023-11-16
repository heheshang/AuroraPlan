use crate::model::client::service::_ds_user_service_client;
use aurora_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::Result,
};
use aurora_proto::ds_user::{DsUser, GetDsUserByIdRequest, QueryUserByNamePasswordRequest};
use tracing::{error, info};

pub async fn query_user_by_name_password(
    user_name: String,
    user_password: String,
    extra: String,
) -> Result<DsUser> {
    info!(
        "query_user_by_name_password user_name: {:?} ,user_password: {:?},extra: {:?}",
        user_name, user_password, extra
    );
    let client = _ds_user_service_client().await?;
    let digest = md5::compute(user_password);
    let request = tonic::Request::new(QueryUserByNamePasswordRequest {
        user_name,
        user_password: format!("{:x}", digest),
    });
    client
        .clone()
        .query_user_by_name_password(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            error!("query_user_by_name_password error: {:?}", e);
            Error::UserNamePasswdError(AuroraData::Null)
        })
}

pub async fn _get_user(id: i32) -> Result<DsUser> {
    let client = _ds_user_service_client().await?;
    let request = tonic::Request::new(GetDsUserByIdRequest { id });
    client
        .clone()
        .get_ds_user_by_id(request)
        .await
        .map(|res| res.into_inner().ds_user.unwrap())
        .map_err(|e| {
            error!("get_user_by_id error: {:?}", e);
            Error::UserNotExist(AuroraData::Null)
        })
}
