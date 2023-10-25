use crate::get_dao_config_path;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Twitter {
    pub consumer_token: String,
    pub consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Braintree {
    pub merchant_id: String,
    pub pub_key: String,
    pub private_key: String,
}
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Security {
    pub authentication_type: Option<String>,
    pub authentication_ldap_user_admin: Option<String>,
    pub authentication_ldap_urls: Option<String>,
    pub authentication_ldap_base_dn: Option<String>,
    pub authentication_ldap_username: Option<String>,
    pub authentication_ldap_password: Option<String>,
    pub authentication_ldap_user_identity_attribute: Option<String>,
    pub authentication_ldap_user_email_attribute: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let config_path = get_dao_config_path();
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(
                config_path.join("default").to_str().unwrap(),
            ))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(config_path.join(run_mode).to_str().unwrap()).required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(
                File::with_name(config_path.join("local").to_str().unwrap()).required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
#[cfg(test)]
mod tests {

    use config::{Config, Environment, File};

    use std::env;

    use crate::get_dao_config_path;

    use super::*;
    #[test]
    fn config_is_work() {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        // get dolphin-api-config path from env
        let config_path = get_dao_config_path();
        eprintln!("config_path: {:?}", config_path);
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(
                config_path.join("default").to_str().unwrap(),
            ))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(config_path.join(run_mode).to_str().unwrap()).required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(
                File::with_name(config_path.join("local").to_str().unwrap()).required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")
            .build()
            .unwrap();

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        let config = s.try_deserialize::<Settings>();
        eprintln!("config{:?}", config);
    }
}
