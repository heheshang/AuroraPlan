use aurora_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::Result,
};
use aurora_proto::{
    ds_session::{
        CreateDsSessionRequest, DeleteDsSessionRequest, GetDsSessionByIdRequest,
        GetDsSessionUserIdRequest, UpdateDsSessionRequest,
    },
    pb::{ds_session::DsSession, ds_user::DsUser},
};
use tracing::{error, info};

use crate::model::client::service::_ds_session_service_client;

pub async fn create_ds_session(user: &DsUser, extra: &str) -> Result<DsSession> {
    let client = _ds_session_service_client().await?;
    info!("user: {:?}, extra: {}", user, extra);
    let user_id = user.id;
    let sessions = get_ds_session_by_user_id(user_id).await?;
    let count = sessions.len();
    for (index, session) in sessions.iter().enumerate() {
        if index > 0 {
            let id = session.id.clone();
            delete_ds_session_by_id(id).await?;
        }
    }

    if count == 0 {
        let ds_session = DsSession {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            ip: Some(extra.to_string()),
            last_login_time: Some("2024-09-11 12:12:12".to_string()),
        };
        let request = tonic::Request::new(CreateDsSessionRequest {
            ds_session: Some(ds_session),
        });
        client
            .clone()
            .create_ds_session(request)
            .await
            .map(|res| res.into_inner())
            .map_err(|e| {
                let err: Error = e.into();
                error!("get_ds_session_by_id error: {:?}", err);
                err
            })
    } else {
        let session = sessions.first().unwrap();
        update_ds_session(session.clone()).await?;
        Ok(DsSession {
            id: session.id.clone(),
            user_id: session.user_id,
            ip: session.ip.clone(),
            last_login_time: session.last_login_time.clone(),
        })
    }
}

pub async fn delete_ds_session_by_id(session_id: String) -> Result<()> {
    let client = _ds_session_service_client().await?;
    let request = tonic::Request::new(DeleteDsSessionRequest {
        id: session_id.clone(),
    });
    client
        .clone()
        .delete_ds_session(request)
        .await
        .map(|_| ())
        .map_err(|e| {
            let err: Error = e.into();
            error!("delete_ds_session_by_id error: {:?}", err);
            err
        })
}

pub async fn _get_session(session_id: String) -> Result<DsSession> {
    let client = _ds_session_service_client().await?;
    let request = tonic::Request::new(GetDsSessionByIdRequest {
        id: session_id.clone(),
    });
    client
        .clone()
        .get_ds_session_by_id(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("get_ds_session_by_id error: {:?}", err);
            err
        })
}

pub async fn get_ds_session_by_user_id(user_id: i32) -> Result<Vec<DsSession>> {
    let client = _ds_session_service_client().await?;
    let request = tonic::Request::new(GetDsSessionUserIdRequest { user_id });
    client
        .clone()
        .get_ds_session_by_user_id(request)
        .await
        .map(|res| res.into_inner().ds_sessions)
        .map_err(|e| {
            error!("get_ds_session_by_user_id error: {:?}", e);
            Error::LoginSessionFailed(AuroraData::Null)
        })
}

pub async fn update_ds_session(ds_session: DsSession) -> Result<()> {
    let client = _ds_session_service_client().await?;
    let request = tonic::Request::new(UpdateDsSessionRequest {
        ds_session: Some(ds_session),
    });
    client
        .clone()
        .update_ds_session(request)
        .await
        .map(|_| ())
        .map_err(|e| {
            error!("update_ds_session error: {:?}", e);
            Error::LoginSessionFailed(AuroraData::Null)
        })
}
