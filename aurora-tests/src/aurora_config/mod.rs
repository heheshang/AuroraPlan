#[cfg(test)]
mod tests {
    use aurora_config::get_api_config_path;
    use aurora_config::get_dao_config_path;
    use config::{Config, Environment, File};
    use std::env;

    #[test]
    fn test_ptah_caculate() {
        let current_dir = env::current_dir().unwrap();

        println!("current_dir: {:?}", current_dir);

        let mut dirs = current_dir.to_str().unwrap().split('/').collect::<Vec<_>>();

        dirs.reverse();

        while !dirs.is_empty() && dirs[0] != "AuroraPlan" {
            dirs.remove(0);
        }

        let path = dirs.iter().rev().copied().collect::<Vec<_>>().join("/");

        println!("final path: {:?}", path);
    }

    #[test]
    fn dao_config_is_work() {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        // get Aurora-api-config path from env
        let config_path = get_dao_config_path();
        println!("config_path: {:?}", config_path);
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(
                config_path.join("default.toml").to_str().unwrap(),
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
        let config = s.try_deserialize::<aurora_config::dao_config::Settings>();
        println!("config{:?}", config);
    }

    #[test]
    fn api_config_is_work() {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        // get Aurora-api-config path from env
        let config_path = get_api_config_path();
        println!("config_path: {:?}", config_path);
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(
                config_path.join("default.toml").to_str().unwrap(),
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
        let config = s.try_deserialize::<aurora_config::api_config::Settings>();
        println!("config{:?}", config);
    }
}
