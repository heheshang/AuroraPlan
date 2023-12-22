use lib_proto::ds_project::DsProject;
use lib_proto::ds_project::DsProjectListRes;
use lib_proto::ds_project::ListDsProjectsResponse;
use lib_proto::ds_project_parameter::ListProjectParametersResponse;
use lib_proto::ds_project_parameter::ProjectParameter;
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
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsProjectsResponse")]
pub struct DsProjectList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<DsProjectListInfo>,
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

#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ProjectParameter")]
pub struct DsProjectParamterRes {
    pub id: i32,
    pub param_name: String,
    pub param_value: String,
    pub code: i64,
    pub project_code: i64,
    pub user_id: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListProjectParametersResponse")]
pub struct ProjectParameterList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<DsProjectParamterRes>,
}
