use lib_proto::ds_plugin_define::DsPluginDefine;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;

#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsPluginDefine")]
pub struct UiPlugin {
    pub id: i32,
    pub plugin_name: String,
    pub plugin_type: String,
    pub plugin_params: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsPluginDefine")]
pub struct UiPluginNoParams {
    pub id: i32,
    pub plugin_name: String,
    pub plugin_type: String,
    #[serde(skip)]
    pub plugin_params: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
