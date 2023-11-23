use super::dao_service::AuroraRpcServer;
use aurora_common::{
    core_error::error::{AuroraData, AuroraErrorInfo, Error},
    core_results::results::Result,
};
use entity::{
    t_ds_environment::{ActiveModel, Column, Entity, EnvironmentToGroupLink},
    t_ds_environment_worker_group_relation,
};
use proto::ds_environment::{ds_environment_service_server::DsEnvironmentService, DsEnvironment};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use tracing::{error, info};
pub struct DsEnvironmentServiceServer(AuroraRpcServer);
impl DsEnvironmentServiceServer {
    async fn env_name_exist(&self, _env_name: &str) -> Result<bool> {
        let db = &self.0.db;
        Ok(Entity::find()
            .filter(Column::Name.eq(_env_name))
            .one(db)
            .await
            .map(|res| res.is_some())
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?)
    }
}

#[tonic::async_trait]
impl DsEnvironmentService for AuroraRpcServer {
    async fn list_ds_environments(
        &self,
        _req: tonic::Request<proto::ds_environment::ListDsEnvironmentsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::ListDsEnvironmentsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::GetDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn create_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::CreateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        let db = &self.db;
        let env_name = _req.get_ref().clone().name;
        let config = _req.get_ref().clone().config;
        let description = _req.get_ref().clone().description;
        let worker_groups = _req.get_ref().clone().worker_groups;
        let operator = _req.get_ref().clone().operator;

        info!("request: {:?}", _req);
        let res = db
            .transaction::<_, DsEnvironment, DbErr>(|tx| {
                Box::pin(async move {
                    let code = aurora_common::utils::code_generate_utils::gen_code().unwrap_or_default();
                    let _ = ActiveModel {
                        id: NotSet,
                        code: Set(code),
                        name: Set(Some(env_name)),
                        config: Set(Some(config)),
                        description: Set(description),
                        operator: Set(Some(operator)),
                        create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
                        update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
                    }
                    .insert(tx)
                    .await;
                    let _ = t_ds_environment_worker_group_relation::Entity::insert_many(
                        worker_groups
                            .split(',')
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|worker_group| t_ds_environment_worker_group_relation::ActiveModel {
                                id: NotSet,
                                operator: Set(Some(operator)),
                                environment_code: Set(code),
                                worker_group: Set(worker_group.to_string()),
                                create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
                                update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
                            })
                            .collect::<Vec<t_ds_environment_worker_group_relation::ActiveModel>>(),
                    )
                    .exec(tx)
                    .await;
                    Entity::find()
                        .find_also_linked(EnvironmentToGroupLink)
                        .filter(Column::Code.eq(code))
                        .one(tx)
                        .await
                        .map(|res| {
                            res.map(|(l, r)| DsEnvironment {
                                id: l.id,
                                code: l.code,
                                name: l.name,
                                config: l.config,
                                description: l.description,
                                operator: l.operator,
                                create_time: Some(l.create_time.unwrap_or_default().to_string()),
                                update_time: Some(l.update_time.unwrap_or_default().to_string()),
                                worker_groups: Some(
                                    r.into_iter().map(|r| r.worker_group).collect::<Vec<String>>().join(","),
                                ),
                            })
                            .unwrap_or_default()
                        })
                })
            })
            .await
            .map_err(|_e| {
                error!("create environment error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?;
        // info!("environment  res: {:?}", res);
        Ok(tonic::Response::new(res))
    }

    async fn update_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::UpdateDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironment>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_environment(
        &self,
        _req: tonic::Request<proto::ds_environment::DeleteDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn verify_ds_environment(
        &self,
        _request: tonic::Request<proto::ds_environment::VerifyDsEnvironmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let env_name = _request.into_inner().environment_name;
        let res = DsEnvironmentServiceServer(self.clone()).env_name_exist(&env_name).await?;
        match res {
            false => Ok(tonic::Response::new(())),
            true => Err(tonic::Status::from_error(
                Error::EnvironmentNameExists(AuroraData::Null, Some(vec![env_name])).into(),
            )),
        }
    }
}
