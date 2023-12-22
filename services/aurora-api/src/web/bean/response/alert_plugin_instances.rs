use lib_proto::ds_alert_plugin_instance::DsAlertPluginInstance;
use lib_proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsAlertPluginInstance")]
pub struct AlertPluginInstance {
    pub id: i32,
    pub plugin_define_id: i32,
    pub plugin_instance_params: ::core::option::Option<String>,
    pub create_time: ::core::option::Option<String>,
    pub update_time: ::core::option::Option<String>,
    pub instance_name: ::core::option::Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsAlertPluginInstancesResponse")]
pub struct AlertPluginInstanceList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<AlertPluginInstance>,
}
