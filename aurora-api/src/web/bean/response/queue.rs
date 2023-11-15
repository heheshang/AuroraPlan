use aurora_proto::ds_queue::{DsQueue, ListDsQueuesResponse};
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsQueue")]
pub struct Queue {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsQueuesResponse")]
pub struct QueueList {
    pub total: u64,
    pub total_page: u64,
    pub page_size: u64,
    pub current_page: u64,
    pub start: u64,
    pub total_list: Vec<Queue>,
}
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsQueue")]
pub struct VerifyQueue {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
