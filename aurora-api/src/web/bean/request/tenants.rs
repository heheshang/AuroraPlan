use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenant {
    pub tenant_code: String,
    pub description: Option<String>,
    pub queue_id: Option<i32>,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct TenantListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTenant {
    pub description: String,
    pub queue_id: i32,
}
