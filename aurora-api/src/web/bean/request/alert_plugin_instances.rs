use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertPluginInstance {
    pub(crate) instance_name: String,
    pub(crate) alert_plugin_instance_id: i32,
    pub(crate) plugin_instance_params: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlertPluginInstance {
    pub(crate) instance_name: String,
    pub(crate) alert_plugin_instance_id: i32,
    pub(crate) plugin_instance_params: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAlertPluginInstance {
    pub(crate) instance_name: String,
}
