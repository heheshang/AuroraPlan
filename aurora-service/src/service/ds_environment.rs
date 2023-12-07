use super::dao_service::AuroraRpcServer;
use aurora_common::{
    core_error::error::{AuroraData, AuroraErrorInfo, Error},
    core_results::results::Result,
};
use entity::{
    t_ds_environment::{ActiveModel, Column, Entity},
    t_ds_environment_worker_group_relation,
    v_ds_environment::Entity as VEntity,
};
use log::{error, info};
use proto::ds_environment::{ds_environment_service_server::DsEnvironmentService, DsEnvironmentPage};
use sea_orm::{
    debug_print, ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    Set, TransactionTrait,
};
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
        let page_num = _req.get_ref().clone().page_num;
        let page_size = _req.get_ref().clone().page_size;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let _db = &self.db;
        let pages = VEntity::find()
            .filter(Column::Name.like(format!("%{}%", search_val)))
            .paginate(&self.db, page_size.try_into().unwrap());
        debug_print!("query sql:{:#?}", pages);
        let page_num = match page_num {
            0 => 0,
            _ => page_num - 1,
        };
        let items = match pages.fetch_page(page_num).await {
            Ok(items) => items,
            Err(_) => {
                error!("fetch_page ds_envrionment error");
                vec![]
            }
        };
        let current_page = pages.cur_page();
        info!("current_page: {}", current_page);
        let (total, total_page) = match pages.num_items_and_pages().await {
            Ok(v) => (v.number_of_items, v.number_of_pages),
            Err(_) => {
                error!("num_items_and_pages ds_envrionment error");
                (0, 0)
            }
        };
        info!("total: {}, total_page: {}", total, total_page);
        let start = (page_num) * page_size;
        info!("start: {}", start);
        let res = proto::ds_environment::ListDsEnvironmentsResponse {
            total: total.try_into().unwrap(),
            page_size,
            total_list: items
                .into_iter()
                .map(|v| {
                    info!("v: {:#?} ", v);

                    DsEnvironmentPage {
                        id: v.id,
                        name: v.name,
                        code: v.code,
                        operator: v.operator,
                        description: v.description,
                        worker_groups: v.worker_groups,
                        config: v.config,
                        create_time: Some(v.create_time.unwrap().to_string()),
                        update_time: Some(v.update_time.unwrap().to_string()),
                    }
                })
                .collect(),
            current_page: current_page + 1,
            start,
            total_page: total_page.try_into().unwrap(),
        };
        Ok(tonic::Response::new(res))
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
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironmentPage>, tonic::Status> {
        let db = &self.db;
        let env_name = _req.get_ref().clone().name;
        let config = _req.get_ref().clone().config;
        let description = _req.get_ref().clone().description;
        let worker_groups = _req.get_ref().clone().worker_groups;
        let operator = _req.get_ref().clone().operator;

        info!("request: {:?}", _req);
        let res = db
            .transaction::<_, DsEnvironmentPage, DbErr>(|tx| {
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
                    VEntity::find().filter(Column::Code.eq(code)).one(tx).await.map(|res| {
                        res.map(|l| DsEnvironmentPage {
                            id: l.id,
                            code: l.code,
                            name: l.name,
                            config: l.config,
                            description: l.description,
                            operator: l.operator,
                            create_time: Some(l.create_time.unwrap_or_default().to_string()),
                            update_time: Some(l.update_time.unwrap_or_default().to_string()),
                            worker_groups: l.worker_groups,
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
        let db = &self.db;
        let code = _req.get_ref().clone().code;

        db.transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                let _res = Entity::delete_many().filter(Column::Code.eq(code)).exec(tx).await.map_err(|_| {
                    tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into())
                });
                let _res = t_ds_environment_worker_group_relation::Entity::delete_many()
                    .filter(t_ds_environment_worker_group_relation::Column::EnvironmentCode.eq(code))
                    .exec(tx)
                    .await
                    .map_err(|_| {
                        tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into())
                    });
                Ok(())
            })
        })
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
