use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DefineUserCountParams {
    projectCode: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TaskStateCountParams {
    startDate: String,
    endDate: String,
    projectCode: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ProcessStateCountParams {
    startDate: String,
    endDate: String,
    projectCode: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ProjectListParams {
    pageSize: u32,
    pageNo: String,
    searchVal: Option<String>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectCreateParams {
    pub projectName: String,
    pub description: Option<String>,
    pub userName: String,
}
