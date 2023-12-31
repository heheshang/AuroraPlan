pub(crate) mod alert_groups;
pub(crate) mod alert_plugin_instances;
pub mod client;
pub(crate) mod cluster;
pub(crate) mod environment;
mod error;
pub(crate) mod projects;
pub(crate) mod queues;
pub(crate) mod session;
pub(crate) mod tenants;
pub(crate) mod ui_plugins;
pub(crate) mod user;
pub(crate) mod worker_groups;
use async_trait::async_trait;
use lib_common::core_results::results::Result;
#[async_trait]
pub trait BaseCrud {
    async fn create(&self) -> Result<Box<Self>> {
        todo!()
    }
    async fn update(&self) -> Result<Box<Self>> {
        todo!()
    }
    async fn delete(&self) -> Result<Box<Self>> {
        todo!()
    }
    async fn list(&self) -> Result<Box<Self>> {
        todo!()
    }
}
