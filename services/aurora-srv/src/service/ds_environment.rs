use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_environment;
use crate::models::t_ds_environment_relation;
use lib_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use lib_proto::ds_environment::{self, ds_environment_service_server::DsEnvironmentService, DsEnvironmentPage};
use tracing::{error, info};

#[tonic::async_trait]
impl DsEnvironmentService for AuroraRpcServer {
    async fn list_ds_environments(
        &self,
        _req: tonic::Request<lib_proto::ds_environment::ListDsEnvironmentsRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_environment::ListDsEnvironmentsResponse>, tonic::Status>
    {
        let page_num = _req.get_ref().clone().page_num;
        let page_size = _req.get_ref().clone().page_size;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let pool = &self.pool;
        let (items, total_page, total, start, cur_page) =
            t_ds_environment_relation::EnvironmentRelation::page(&search_val, page_num, page_size, pool)
                .await
                .map_err(|_e| {
                    error!("list environment error: {:?}", _e);
                    tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                        Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                    ))
                })?;
        let res = lib_proto::ds_environment::ListDsEnvironmentsResponse {
            total,
            page_size,
            total_list: items
                .into_iter()
                .map(|v| {
                    info!("v: {:#?} ", v);

                    DsEnvironmentPage {
                        id: v.id.unwrap_or_default(),
                        name: v.name,
                        code: v.code.unwrap_or_default(),
                        operator: v.operator,
                        description: v.description,
                        worker_groups: v.worker_groups.unwrap_or_default().into_iter().collect(),
                        config: v.config,
                        create_time: Some(v.create_time.unwrap().to_string()),
                        update_time: Some(v.update_time.unwrap().to_string()),
                    }
                })
                .collect(),
            current_page: cur_page,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

    async fn get_ds_environment(
        &self,
        _req: tonic::Request<lib_proto::ds_environment::GetDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn create_ds_environment(
        &self,
        _req: tonic::Request<lib_proto::ds_environment::CreateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_environment::DsEnvironmentPage>, tonic::Status> {
        let pool = &self.pool;
        let env_name = _req.get_ref().clone().name;
        let config = _req.get_ref().clone().config;
        let description = _req.get_ref().clone().description;
        let worker_groups = _req.get_ref().clone().worker_groups;
        let operator = _req.get_ref().clone().operator;

        info!("request: {:?}", _req);
        let res = t_ds_environment::Model::create_and_relation(
            &env_name,
            &config,
            description,
            operator,
            worker_groups.clone(),
            pool,
        )
        .await
        .map_err(|_e| {
            error!("create environment error: {:?}", _e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })?;
        let res = lib_proto::ds_environment::DsEnvironmentPage {
            id: res.id,
            name: res.name,
            code: res.code,
            operator: res.operator,
            description: res.description,
            worker_groups,
            config: res.config,
            create_time: Some(res.create_time.unwrap().to_string()),
            update_time: Some(res.update_time.unwrap().to_string()),
        };
        Ok(tonic::Response::new(res))
    }

    async fn update_ds_environment(
        &self,
        _req: tonic::Request<lib_proto::ds_environment::UpdateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_environment::DsEnvironment>, tonic::Status> {
        Err(tonic::Status::unimplemented("not implemented"))
    }

    async fn delete_ds_environment(
        &self,
        _req: tonic::Request<lib_proto::ds_environment::DeleteDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let code = _req.get_ref().clone().code;
        t_ds_environment::Model::delete_by_code_and_relation(code, pool)
            .await
            .map_err(|_e| {
                error!("delete environment error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?;

        Ok(tonic::Response::new(()))
    }

    async fn verify_ds_environment(
        &self,
        _request: tonic::Request<lib_proto::ds_environment::VerifyDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let name = _request.get_ref().clone().environment_name;
        let pool = &self.pool;
        let res = t_ds_environment::Model::env_name_exist(name.as_str(), pool)
            .await
            .map_err(|_e| {
                error!("verify environment error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?;
        if res {
            let err = Error::EnvironmentNameExists(AuroraData::Null, Some(vec![name]));
            error!("err: {:?}", err);
            let status = tonic::Status::from_error(Box::<AuroraErrorInfo>::new(err.into()));
            error!("status: {:?}", status);
            Err(status)
        } else {
            Ok(tonic::Response::new(()))
        }
    }

    async fn all_ds_environments(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_environment::AllDsEnvironmentsResponse>, tonic::Status> {
        let pool = &self.pool;
        let res = t_ds_environment_relation::EnvironmentRelation::all(pool)
            .await
            .map_err(|_e| {
                error!("all environment error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?
            .into_iter()
            .map(|v| {
                let worker_groups = v.worker_groups.unwrap_or_default().into_iter().collect();
                ds_environment::DsEnvironmentPage {
                    id: v.id.unwrap_or_default(),
                    name: v.name,
                    code: v.code.unwrap_or_default(),
                    operator: v.operator,
                    description: v.description,
                    worker_groups,
                    config: v.config,
                    create_time: Some(v.create_time.unwrap().to_string()),
                    update_time: Some(v.update_time.unwrap().to_string()),
                }
            })
            .collect();
        Ok(tonic::Response::new(
            lib_proto::ds_environment::AllDsEnvironmentsResponse { total_list: res },
        ))
    }
}
