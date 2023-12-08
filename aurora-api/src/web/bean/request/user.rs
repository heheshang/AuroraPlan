use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoReq {
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginInfoReq {
    pub user_name: String,
    pub user_password: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    pub queue: Option<String>,
    pub state: Option<i32>,
}
