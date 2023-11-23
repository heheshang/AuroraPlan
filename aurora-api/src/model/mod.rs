pub mod client;
pub(crate) mod environment;
mod error;
pub(crate) mod projects;
pub(crate) mod queues;
pub(crate) mod session;
pub(crate) mod tentans;
pub(crate) mod user;
use async_trait::async_trait;
use aurora_common::core_results::results::Result;
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
