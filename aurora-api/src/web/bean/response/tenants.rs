use aurora_proto::ds_tenant::DsTenant;
use aurora_proto::ds_tenant::ListDsTenantsResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsTenant")]
pub struct Tenant {
    pub id: i32,
    pub tenant_code: Option<String>,
    pub description: Option<String>,
    pub queue_id: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsTenantsResponse")]
pub struct TenantList {
    pub total: u64,
    pub total_page: u64,
    pub page_size: u64,
    pub current_page: u64,
    pub start: u64,
    pub total_list: Vec<Tenant>,
}
