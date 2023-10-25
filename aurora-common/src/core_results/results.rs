use crate::core_error::error::{AuroraErrorInfo, Error};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;
pub type GrpcResponse<T> = std::result::Result<tonic::Response<T>, tonic::Status>;
pub type GrpcRequest<T> = tonic::Request<T>;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct RespResult<T>(pub Result<T>);

// impl<T> IntoResponse for RespResult<T>
// where T: Serialize
// {
//     fn into_response(self) -> axum::response::Response {
//         match self.0 {
//             Ok(_) => todo!(),
//             Err(_) => todo!(),
//         }
//     }
// }
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResult<T> {
    pub data: Option<T>,
    pub code: i32,
    pub msg: String,
    pub failed: bool,
    pub success: bool,
}

impl<T> ApiResult<T> {
    pub fn build(data: Option<T>) -> Self {
        let errmsg = AuroraErrorInfo::default();
        Self {
            data,
            code: errmsg.code,
            msg: errmsg.cn_msg,
            failed: false,
            success: true,
        }
    }
}

impl<T> Default for ApiResult<T> {
    fn default() -> Self {
        let errmsg = AuroraErrorInfo::default();
        Self {
            data: None,
            code: errmsg.code,
            failed: false,
            success: true,
            msg: errmsg.cn_msg,
        }
    }
}

impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);
        body.into_response()
    }
}
// #[warn(dead_code)]
// fn format_args(text: &str, args: Vec<String>) -> String {
//     let mut new_text = text.to_string();
//     let re = regex::Regex::new(r"\{(\d+)").unwrap();
//     for cap in re.captures_iter(text) {
//         let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
//         if args.len() <= index {
//             continue;
//         }
//         let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
//         new_text = ss.clone();
//     }
//     new_text
// }
#[cfg(test)]
mod tests {

    #[test]
    fn regex_is_work() {
        let text = "copy process definition from {0} to {2} error : {1}";
        let mut new_text = text.to_string();
        let args = vec![String::from("aaa"), String::from("bb"), String::from("cc")];

        let re = regex::Regex::new(r"\{(\d+)").unwrap();
        // let mut result = String::new();

        for cap in re.captures_iter(text) {
            let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", index);
            if args.len() <= index {
                continue;
            }
            let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
            new_text = ss.clone();
            // println!("{}", ss);
            // result.push_str(&args[index]);
        }

        println!("{}", new_text);
    }
}
