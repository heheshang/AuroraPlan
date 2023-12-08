use serde::Deserialize;

pub mod environment;
pub mod projects;
pub mod queue;
pub mod tenants;
pub mod user;
pub mod worker_groups;
#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct PageParams {
    pub pageSize: i64,
    pub pageNo: i64,
    pub searchVal: Option<String>,
}
