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
#[derive(Deserialize, Clone, Debug)]
pub struct ProjectListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectCreateParams {
    pub projectName: String,
    pub description: Option<String>,
    pub userName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectParamCreate {
    pub projectParameterName: String,
    pub projectParameterValue: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct ProjectParameterListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
    pub projectCode: u64,
}
