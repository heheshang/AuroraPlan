use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UiPlugin {
    pub id: i32,
    pub plugin_name: String,
    pub plugin_type: String,
    pub plugin_params: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPluginQueryByType {
    pub plugin_type: String,
}
