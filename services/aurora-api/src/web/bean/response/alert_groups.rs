use lib_proto::ds_alertgroup::DsAlertGroup;
use lib_proto::ds_alertgroup::ListDsAlertGroupsResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsAlertGroup")]
pub struct AlertGroup {
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsAlertGroupsResponse")]
pub struct AlertGroupList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<AlertGroup>,
}
