use crate::model::{
    session::service as session_service,
    user::{self, service as user_service},
};
use async_trait::async_trait;
use aurora_common::core_results::results::Result;
use aurora_proto::pb::ds_user::DsUser;
use std::collections::HashMap;
use tracing::info;
pub enum AuthenticatorType {
    Password,
    Ladp,
}

impl AuthenticatorType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "PASSWORD" => AuthenticatorType::Password,
            "LADP" => AuthenticatorType::Ladp,
            _ => AuthenticatorType::Password,
        }
    }
}

#[async_trait]
pub trait Authenticator: Sync + Send {
    async fn login(&self, username: String, password: String, extra: String) -> Result<DsUser>;
    async fn authenticate(
        &self,
        username: String,
        password: String,
        extra: String,
    ) -> Result<HashMap<String, String>> {
        let data = self.login(username, password, extra.clone()).await?;
        let session = session_service::create_ds_session(&data, &extra).await?;
        let mut map = HashMap::new();
        map.insert("sessionId".to_string(), session.id);
        return Ok(map);
    }
    async fn get_auth_user(&self, session_id: String) -> Result<DsUser> {
        let sesion_res = session_service::_get_session(session_id).await?;
        user::service::_get_user(sesion_res.user_id).await
    }
}

#[derive(Default)]
pub struct PasswordAuthenticator;
#[derive(Default)]
pub struct LdapAuthenticator;

#[async_trait]
impl Authenticator for PasswordAuthenticator {
    async fn login(&self, username: String, password: String, extra: String) -> Result<DsUser> {
        info!(
            "username:{},password:{},extra:{}",
            username, password, extra
        );
        user_service::query_user_by_name_password(username, password, extra.clone()).await
    }
}

#[async_trait]
impl Authenticator for LdapAuthenticator {
    async fn login(&self, username: String, password: String, extra: String) -> Result<DsUser> {
        info!(
            "username:{},password:{},extra:{}",
            username, password, extra
        );
        todo!()
    }
}
