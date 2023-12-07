use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DefineUserCountParams {
    projectCode: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TaskStateCountParams {
    startDate: String,
    endDate: String,
    projectCode: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ProcessStateCountParams {
    startDate: String,
    endDate: String,
    projectCode: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct ProjectListParams {
    pub pageSize: i64,
    pub pageNo: i64,
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
    pub pageSize: i64,
    pub pageNo: i64,
    pub searchVal: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectParamUpdate {
    pub code: i64,
    pub projectParameterName: String,
    pub projectParameterValue: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectParamDelete {
    pub code: i64,
}
