use aurora_proto::ds_project::DsProject;
use aurora_proto::ds_project::DsProjectListRes;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsProject")]
pub struct DsProjectRes {
    pub id: i32,
    pub name: Option<String>,
    pub code: i64,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub flag: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DsProjectList {
    pub total: i32,
    pub total_page: i32,
    pub page_size: i32,
    pub current_page: i32,
    pub start: i32,
    pub total_list: Vec<DsProjectRes>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsProjectListRes")]
pub struct DsProjectListInfo {
    pub id: i32,
    pub name: Option<String>,
    pub code: i64,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub flag: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub user_name: String,
}
