use lib_proto::ds_cluster::DsCluster;
use lib_proto::ds_cluster::ListDsClustersResponse;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;
#[derive(Serialize, Deserialize, Debug, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "DsCluster")]
pub struct Cluster {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from = "ListDsClustersResponse")]
pub struct ClusterList {
    pub total: i64,
    pub total_page: i64,
    pub page_size: i64,
    pub current_page: i64,
    pub start: i64,
    pub total_list: Vec<Cluster>,
}
