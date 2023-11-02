// Purpose: Module file for the auth module.
pub mod authenticator;
pub(crate) mod error;

use std::str::FromStr;

use aurora_config::api_config::Settings;

use self::authenticator::{
    Authenticator, AuthenticatorType, LdapAuthenticator, PasswordAuthenticator,
};

trait AuthenticatorFactory {
    fn build() -> Box<dyn Authenticator>;
}

struct PwdAuthenticatorFactory;
struct LdapAuthenticatorFactory;

impl AuthenticatorFactory for LdapAuthenticatorFactory {
    fn build() -> Box<dyn Authenticator> {
        Box::<LdapAuthenticator>::default()
    }
}
impl AuthenticatorFactory for PwdAuthenticatorFactory {
    fn build() -> Box<dyn Authenticator> {
        Box::<PasswordAuthenticator>::default()
    }
}

pub fn get_authenticator() -> Box<dyn Authenticator> {
    let settings = Settings::new().unwrap();
    let auth_type = settings
        .security
        .authentication_type
        .unwrap_or("PASSWORD".to_string());
    let r#type = AuthenticatorType::from_str(auth_type.as_str()).unwrap();
    match r#type {
        AuthenticatorType::Password => PwdAuthenticatorFactory::build(),
        AuthenticatorType::Ladp => LdapAuthenticatorFactory::build(),
    }
}
