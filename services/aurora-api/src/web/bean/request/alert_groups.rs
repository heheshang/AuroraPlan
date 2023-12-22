use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertGroup {
    pub(crate) group_name: String,
    pub(crate) alert_instance_ids: String,
    pub(crate) description: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlertGroup {
    pub(crate) group_name: String,
    pub(crate) alert_instance_ids: String,
    pub(crate) description: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAlertGroup {
    pub(crate) group_name: String,
}
