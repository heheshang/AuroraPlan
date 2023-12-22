use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCluster {
    pub(crate) name: String,
    pub(crate) config: String,
    pub(crate) description: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCluster {
    pub(crate) code: i64,

    pub(crate) name: String,
    pub(crate) config: String,
    pub(crate) description: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCluster {
    pub(crate) cluster_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCluster {
    pub(crate) cluster_code: i64,
}
