use crate::models::t_ds_project::Model;

use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, Error};
use aurora_common::utils::code_generate_utils::gen_code;
use proto::ds_project::ds_project_service_server::DsProjectService;
use tracing::info;

#[tonic::async_trait]
impl DsProjectService for AuroraRpcServer {
    async fn list_ds_projects(
        &self,
        _req: tonic::Request<proto::ds_project::ListDsProjectsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::ListDsProjectsResponse>, tonic::Status> {
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{}",
            search_val, page_size, page_num
        );
        let res = Model::page(&search_val, page_num, page_size, pool)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;

        let res = proto::ds_project::ListDsProjectsResponse {
            total: res.2,
            page_size,
            total_list: res.0.into_iter().map(|v| v.into()).collect(),
            current_page: res.4,
            start: res.3,
            total_page: res.1,
        };

        Ok(tonic::Response::new(res))
    }

    async fn get_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::GetDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        todo!()
    }

    async fn create_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::CreateDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        let pool = &self.pool;
        let req = _req.into_inner();

        let code = gen_code().unwrap_or_default();

        Model::create(code, Some(req.name), req.description, Some(req.user_id), pool)
            .await
            .map(|v| Ok(tonic::Response::new(v.into())))
            .map_err(|_| tonic::Status::from_error(Error::CreateProjectError(AuroraData::Null, None).into()))?
    }

    async fn update_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::UpdateDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        let pool = &self.pool;
        let name = _req.get_ref().name.clone();
        let description = _req.get_ref().description.clone();
        let user_id = _req.get_ref().user_id;
        Model::update(Some(name), description, Some(user_id), pool)
            .await
            .map(|v| Ok(tonic::Response::new(v.into())))
            .map_err(|_| tonic::Status::from_error(Error::UpdateProjectError(AuroraData::Null, None).into()))?
    }

    async fn delete_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::DeleteDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        Model::delete(_req.get_ref().id, pool)
            .await
            .map(|_| Ok(tonic::Response::new(())))
            .map_err(|_| tonic::Status::from_error(Error::DeleteProjectError(AuroraData::Null, None).into()))?
    }
}
