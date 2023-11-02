use std::{
    env,
    path::{Path, PathBuf},
};

pub mod api_config;
pub mod dao_config;

pub fn get_api_config_path() -> PathBuf {
    glb_config_dir().join("api-config")
}

pub fn get_dao_config_path() -> PathBuf {
    glb_config_dir().join("dao-config")
    // let value = dotenvy::var("CURRENT_DIR").expect("CURRENT_DIR must be set");

    // Path::new(&value).join("dao-config")
}

fn glb_config_dir() -> PathBuf {
    // 获取当前工作目录
    let value = dotenvy::var("CURRENT_DIR");
    match value {
        Ok(v) => Path::new(&v).join("aurora-config"),
        Err(_) => {
            let current_dir = env::current_dir().unwrap();

            println!("current_dir: {:?}", current_dir);

            let mut dirs = current_dir.to_str().unwrap().split('/').collect::<Vec<_>>();

            dirs.reverse();

            while !dirs.is_empty() && dirs[0] != "AuroraPlan" {
                dirs.remove(0);
            }

            let path = dirs.iter().rev().copied().collect::<Vec<_>>().join("/");

            println!("final path: {:?}", path);

            Path::new(&path).join("aurora-config")
        }
    }
}
