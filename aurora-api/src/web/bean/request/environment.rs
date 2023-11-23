use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironment {
    pub name: String,
    pub config: String,
    pub description: Option<String>,
    pub worker_groups: Vec<String>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct EnvironmentListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnvironment {
    pub code: i64,
    pub name: String,
    pub config: String,
    pub description: Option<String>,
    pub worker_groups: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEnvironment {
    pub environment_name: String,
}
