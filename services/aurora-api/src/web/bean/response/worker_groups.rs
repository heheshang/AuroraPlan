use lib_proto::ds_worker_group::DsWorkerGroup;
use lib_proto::ds_worker_group::ListDsWorkerGroupsResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsWorkerGroup")]
pub struct WorkerGroups {
    pub id: i64,
    pub name: String,
    pub addr_list: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsWorkerGroupsResponse")]
pub struct WorkerGroupsList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<WorkerGroups>,
}
