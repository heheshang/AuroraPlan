use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use entity::t_ds_project::{self, ActiveModel, Column, Entity, ProjectToUserLink};
use proto::ds_project::ds_project_service_server::DsProjectService;
use proto::ds_project::DsProjectListRes;
use sea_orm::{debug_print, Set};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use snowflake::SnowflakeIdBucket;
use tracing::{error, info};

#[tonic::async_trait]
impl DsProjectService for AuroraRpcServer {
    async fn list_ds_projects(
        &self,
        _req: tonic::Request<proto::ds_project::ListDsProjectsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project::ListDsProjectsResponse>,
        tonic::Status,
    > {
        let conn = &self.conn;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{}",
            search_val, page_size, page_num
        );
        let pages = Entity::find()
            .order_by_desc(Column::CreateTime)
            .filter(t_ds_project::Column::Name.like(format!("%{}%", search_val)))
            .find_also_linked(ProjectToUserLink)
            // .find_with_linked(ProjectToUserLink)
            // .join(
            //     sea_orm::JoinType::LeftJoin,
            //     t_ds_project::Relation::TDsProjectUser.def(),
            // )
            // // .select_only()
            // // .select_only()
            // .column_as(Column::Name, Column::Name.to_string())
            // .column_as(Column::Id, Column::Id.to_string())
            // .column_as(Column::Code, Column::Code.to_string())
            // .column_as(Column::Description, Column::Description.to_string())
            // .column(Column::Flag)
            // .column_as(Column::UserId, Column::UserId.to_string())
            // .column_as(Column::CreateTime, Column::CreateTime.to_string())
            // .column_as(Column::UpdateTime, Column::UpdateTime.to_string())
            // .select_column(UserColumn::UserName)
            .paginate(conn, page_size);
        // info!("query sql:{:#?}", pages);
        debug_print!("query sql:{:#?}", pages);
        let page_num = match page_num {
            0 => 0,
            _ => page_num - 1,
        };
        let items = match pages.fetch_page(page_num).await {
            Ok(items) => items,
            Err(_) => {
                error!("fetch_page ds_project error");
                vec![]
            }
        };
        let current_page = pages.cur_page();
        info!("current_page: {}", current_page);
        let (total, total_page) = match pages.num_items_and_pages().await {
            Ok(v) => (v.number_of_items, v.number_of_pages),
            Err(_) => {
                error!("num_items_and_pages ds_project error");
                (0, 0)
            }
        };
        info!("total: {}, total_page: {}", total, total_page);
        let start = (page_num) * page_size;
        info!("start: {}", start);
        let res = proto::ds_project::ListDsProjectsResponse {
            total,
            page_size,
            total_list: items
                .into_iter()
                .map(|(v, u)| {
                    info!("v: {:#?} ", v);
                    info!("u: {:#?} ", u);
                    DsProjectListRes {
                        id: v.id,
                        name: v.name,
                        code: v.code,
                        user_id: v.user_id,
                        description: v.description,
                        flag: v.flag,
                        create_time: Some(v.create_time.unwrap().to_string()),
                        update_time: Some(v.update_time.unwrap().to_string()),
                        user_name: u.unwrap().user_name.unwrap(),
                    }
                })
                .collect(),
            current_page: current_page + 1,
            start,
            total_page,
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
        let conn = &self.conn;
        let req = _req.into_inner();
        let current_time = chrono::prelude::Local::now().naive_local();

        let code = SnowflakeIdBucket::new(1, 1).get_id();
        let res = ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            code: Set(code),
            name: Set(Some(req.name)),
            user_id: Set(Some(req.user_id)),
            description: Set(req.description),
            flag: Set(Some(1)),
            create_time: Set(Some(current_time)),
            update_time: Set(Some(current_time)),
        }
        .insert(conn)
        .await;

        match res {
            Ok(v) => Ok(tonic::Response::new(v.into())),
            Err(_) => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::CreateProjectError(AuroraData::Null).into(),
            ))),
        }
    }

    async fn update_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::UpdateDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::DeleteDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::t_ds_project::Entity;
    use entity::t_ds_project::ProjectToUserLink;
    use sea_orm::Iden;
    use sea_orm::QuerySelect;
    use sea_orm::RelationTrait;
    use sea_orm::{QueryTrait, SelectColumns};
    #[test]
    fn test_link_sql() {
        let sql = Entity::find()
            .find_also_linked(ProjectToUserLink)
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        println!("{}", sql);
        let sql1 = Entity::find()
            .join_rev(
                sea_orm::JoinType::LeftJoin,
                t_ds_project::Relation::TDsProjectUser.def(),
            )
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();
        println!("{}", sql1);
        let sql2 = Entity::find()
            .join_rev(
                sea_orm::JoinType::Join,
                t_ds_project::Relation::TDsProjectUser.def(),
            )
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();
        println!("{}", sql2);
        println!("-------------------");
        let sql3 = Entity::find()
            .join(
                sea_orm::JoinType::LeftJoin,
                t_ds_project::Relation::TDsProjectUser.def(),
            )
            .select_only()
            // .select_also(entity::t_ds_user::Entity)
            // .select_also(entity::t_ds_project::Entity)
            .select_column_as(
                entity::t_ds_project::Column::Name,
                entity::t_ds_project::Column::Name.to_string(),
            )
            .select_column_as(entity::t_ds_user::Column::UserName, "user_name")
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();
        println!("{}", sql3);
    }
}
