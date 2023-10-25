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

            // 获取当前文件名
            let args = env::args().next().unwrap();
            let filename = Path::new(&args).file_name().unwrap().to_str().unwrap();
            // 拼接完整的文件路径
            let full_path = current_dir.join(filename);
            // 将完整路径转换为字符串并打印
            let full_path_string = full_path.to_str().unwrap();
            println!("full_path_string: {:?}", full_path_string);
            let path = full_path_string
                .split("aurora-config")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string();
            println!("path: {:?}", path);
            Path::new(&path).join("aurora-config")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_api_config_path() {
        let path = super::get_dao_config_path();
        println!("path: {:?}", path);
    }
}
