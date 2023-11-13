use crate::model::client::service::_ds_project_service_client;
use aurora_common::{core_error::error::Error, core_results::results::Result};
use aurora_proto::{ds_project::DsProject, pb::ds_project::CreateDsProjectRequest};
use tracing::log::error;
pub async fn create(user_id: i32, name: String, description: Option<String>) -> Result<DsProject> {
    let client = _ds_project_service_client().await?;
    let request = tonic::Request::new(CreateDsProjectRequest {
        name,
        user_id,
        description,
    });
    client
        .clone()
        .create_ds_project(request)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| {
            let err: Error = e.into();
            error!("create_project error: {:?}", err);
            err
        })
}
