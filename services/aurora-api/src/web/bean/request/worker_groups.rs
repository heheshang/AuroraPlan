use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkerGroups {
    pub name: String,
    pub addr_list: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct WorkerGroupsListParams {
    pub pageSize: i64,
    pub pageNo: i64,
    pub searchVal: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkerGroups {
    pub name: String,
    pub addr_list: String,
}
