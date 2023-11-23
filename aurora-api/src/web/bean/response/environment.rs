use aurora_proto::ds_environment::DsEnvironment;
use aurora_proto::ds_environment::ListDsEnvironmentsResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsEnvironment")]
pub struct Environment {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsEnvironmentsResponse")]
pub struct EnvironmentList {
    pub total: u64,
    pub total_page: u64,
    pub page_size: u64,
    pub current_page: u64,
    pub start: u64,
    pub total_list: Vec<Environment>,
}
