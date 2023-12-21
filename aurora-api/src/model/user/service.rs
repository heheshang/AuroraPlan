use crate::{
    model::client::service::_ds_user_service_client,
    web::bean::{request::user::UserInfo, response::user::UserInfoRes},
};
use aurora_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::Result,
};
use aurora_proto::ds_user::{
    CreateDsUserRequest, DsUser, GetDsUserByIdRequest, ListDsUsersRequest, ListDsUsersResponse,
    QueryUserByNamePasswordRequest, QueryUserByNameRequest, UpdateDsUserRequest,
};
use tracing::{error, info};

pub async fn query_user_by_name_password(user_name: String, user_password: String, extra: String) -> Result<DsUser> {
    info!(
        "query_user_by_name_password user_name: {:?} ,user_password: {:?},extra: {:?}",
        user_name, user_password, extra
    );
    let client = _ds_user_service_client().await?;
    let digest = md5::compute(user_password);
    let user_password = format!("{:x}", digest);
    let request = tonic::Request::new(QueryUserByNamePasswordRequest {
        user_name,
        user_password,
    });
    client
        .clone()
        .query_user_by_name_password(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            error!("query_user_by_name_password error: {:?}", e);
            Error::UserNamePasswdError(AuroraData::Null, None)
        })
}
pub async fn verify_user_name(user_name: String) -> Result<()> {
    info!("verify_user_name user_name: {:?}", user_name);
    let client = _ds_user_service_client().await?;
    let request = tonic::Request::new(QueryUserByNameRequest {
        user_name: user_name.clone(),
    });
    let res = client
        .clone()
        .query_user_by_name(request)
        .await
        .map(|res| res.into_inner())
        .map(|res| res.user.is_some_and(|v| v.user_name == Some(user_name.clone())))
        .map_err(|e| {
            error!("verify_user_name error: {:?}", e);
            Error::InternalServerErrorArgs(AuroraData::Null, None)
        })?;
    if res {
        return Err(Error::UserNameExist(AuroraData::Null, Some(vec![user_name])));
    }
    Ok(())
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
            Error::UserNotExist(AuroraData::Null, None)
        })
}
pub async fn list(page_num: &i64, page_size: &i64, search_val: Option<String>) -> Result<ListDsUsersResponse> {
    info!(
        "list tenants page_num: {:?} ,page_size: {:?},search_val: {:?}",
        page_num, page_size, search_val
    );
    let client = _ds_user_service_client().await?;
    let request = tonic::Request::new(ListDsUsersRequest {
        page_num: *page_num,
        page_size: *page_size,
        search_val: search_val.clone(),
    });
    let res = client
        .clone()
        .list_ds_users(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("list tenants error: {:?}", err);
            err
        })?;
    Ok(res)
}

pub async fn update(user: UserInfo) -> Result<()> {
    info!("update user: {:?}", user);
    let client = _ds_user_service_client().await?;
    let request = tonic::Request::new(UpdateDsUserRequest {
        ds_user: Some(DsUser {
            id: user.id.unwrap_or_default(),
            user_name: user.user_name,
            queue: user.queue,
            tenant_id: user.tenant_id,
            email: user.email,
            phone: user.phone,
            state: user.state,
            ..Default::default()
        }),
        ..Default::default()
    });
    client.clone().update_ds_user(request).await.map_err(|e| {
        let err: Error = e.into();
        error!("update user error: {:?}", err);
        err
    })?;
    Ok(())
}

pub async fn create(user: UserInfo) -> Result<UserInfoRes> {
    info!("update user: {:?}", user);
    let client = _ds_user_service_client().await?;
    let digest = md5::compute(user.user_password.unwrap_or_default().clone());
    let user_password = format!("{:x}", digest);
    let request = tonic::Request::new(CreateDsUserRequest {
        ds_user: Some(DsUser {
            user_name: user.user_name,
            queue: user.queue,
            tenant_id: user.tenant_id,
            email: user.email,
            phone: user.phone,
            state: user.state,
            user_password: Some(user_password),
            ..Default::default()
        }),
    });
    let res = client.clone().create_ds_user(request).await.map_err(|e| {
        let err: Error = e.into();
        error!("update user error: {:?}", err);
        err
    })?;
    Ok(res.into_inner().into())
}
