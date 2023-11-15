use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateQueueRequest {
    pub id: i32,
    pub queue: String,
    pub queue_name: String,
}
impl Default for CreateQueueRequest {
    fn default() -> Self {
        Self {
            id: -1,
            queue: "".to_string(),
            queue_name: "".to_string(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct QueueListParams {
    pub pageSize: u64,
    pub pageNo: u64,
    pub searchVal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueue {
    pub id: i32,
    pub queue_name: String,
    pub queue: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
