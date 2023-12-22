use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_project_parameter::Model;
use lib_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use lib_proto::ds_project_parameter::project_parameter_service_server::ProjectParameterService;
use tracing::{error, info};

#[tonic::async_trait]
impl ProjectParameterService for AuroraRpcServer {
    async fn list_project_parameters(
        &self,
        _req: tonic::Request<lib_proto::ds_project_parameter::ListProjectParametersRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_project_parameter::ListProjectParametersResponse>,
        tonic::Status,
    > {
        info!("request: {:?}", _req);
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        let project_code = _req.get_ref().clone().project_code;
        Model::page(&search_val, page_num, page_size, project_code, pool)
            .await
            .map(|(items, total_page, total, start, current_page)| {
                let res = lib_proto::ds_project_parameter::ListProjectParametersResponse {
                    total,
                    page_size,
                    total_list: items.into_iter().map(|v| v.into()).collect(),
                    current_page,
                    start,
                    total_page,
                };
                tonic::Response::new(res)
            })
            .map_err(|e| {
                error!("list_project_parameters error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn get_project_parameter(
        &self,
        _request: tonic::Request<lib_proto::ds_project_parameter::GetProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_project_parameter::ProjectParameter>, tonic::Status> {
        info!("request: {:?}", _request);
        todo!()
    }

    async fn create_project_parameter(
        &self,
        _request: tonic::Request<lib_proto::ds_project_parameter::CreateProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_project_parameter::ProjectParameter>, tonic::Status> {
        info!("request: {:?}", _request);
        let pool = &self.pool;
        let project_parameter = _request.get_ref().project_parameter.clone().unwrap();
        let code = lib_common::utils::code_generate_utils::gen_code().unwrap_or_default();
        let user_id = project_parameter.user_id;
        let project_code = project_parameter.project_code;
        let param_name = project_parameter.param_name;
        let param_value = project_parameter.param_value;

        Model::create(&param_name, &param_value, code, project_code, user_id, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("create_project_parameter error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn update_project_parameter(
        &self,
        _request: tonic::Request<lib_proto::ds_project_parameter::UpdateProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_project_parameter::ProjectParameter>, tonic::Status> {
        info!("request: {:?}", _request);
        let pool = &self.pool;
        let code = _request.get_ref().code;
        let project_code = _request.get_ref().project_code;
        let param_name = _request.get_ref().param_name.clone();
        let param_value = _request.get_ref().param_value.clone();
        Model::update_by_code_and_project_code(&param_name, &param_value, code, project_code, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("create_project_parameter error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn delete_project_parameter(
        &self,
        _request: tonic::Request<lib_proto::ds_project_parameter::DeleteProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        info!("request: {:?}", _request);
        let pool = &self.pool;
        let code = _request.get_ref().code;
        let project_code = _request.get_ref().project_code;
        Model::delete_by_code_and_project_code(code, project_code, pool)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_e| {
                error!("delete_project_parameter error: {:?}", _e);
                Error::InternalServerErrorArgs(AuroraData::Null, None).into()
            })
    }
}
