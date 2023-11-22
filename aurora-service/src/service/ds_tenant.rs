use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::AuroraData;

use aurora_common::core_error::error::AuroraErrorInfo;
use aurora_common::core_error::error::Error;
use entity::t_ds_tenant::{ActiveModel, Column, Entity, Model};
use proto::ds_tenant::ds_tenant_service_server::DsTenantService;
use sea_orm::ActiveModelTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryOrder;
use sea_orm::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::{error, info};

type Result<T> = std::result::Result<T, tonic::Status>;

impl AuroraRpcServer {
    async fn tenant_find_by_code(&self, tenant_code: &str) -> Result<Option<Model>> {
        let conn: &DatabaseConnection = &self.db;
        let res = Entity::find()
            .filter(Column::TenantCode.eq(tenant_code))
            .one(conn)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        Ok(res)
    }
    async fn tenant_code_exist(&self, tenant_code: &str) -> Result<bool> {
        Self::tenant_find_by_code(self, tenant_code).await.map(|res| res.is_some())
    }
    #[allow(dead_code)]
    async fn tenant_find_by_id(&self, id: i32) -> Result<Option<Model>> {
        let conn: &DatabaseConnection = &self.db;
        let res =
            Entity::find().filter(Column::Id.eq(id)).one(conn).await.map_err(|_| {
                tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into())
            })?;
        Ok(res)
    }
}

#[tonic::async_trait]
impl DsTenantService for AuroraRpcServer {
    async fn list_ds_tenants(
        &self,
        _req: tonic::Request<proto::ds_tenant::ListDsTenantsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::ListDsTenantsResponse>, tonic::Status> {
        let db = &self.db;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{}",
            search_val, page_size, page_num
        );
        let pages = Entity::find()
            .order_by_desc(Column::CreateTime)
            .filter(Column::TenantCode.like(format!("%{}%", search_val)))
            .paginate(db, page_size);
        // info!("query sql:{:#?}", pages);
        let page_num = match page_num {
            0 => 0,
            _ => page_num - 1,
        };
        let items = match pages.fetch_page(page_num).await {
            Ok(items) => items,
            Err(_) => {
                error!("fetch_page ds_tenants error");
                vec![]
            }
        };
        let current_page = pages.cur_page();
        info!("current_page: {}", current_page);
        let (total, total_page) = match pages.num_items_and_pages().await {
            Ok(v) => (v.number_of_items, v.number_of_pages),
            Err(_) => {
                error!("num_items_and_pages ds_tenants error");
                (0, 0)
            }
        };
        info!("total: {}, total_page: {}", total, total_page);
        let start = (page_num) * page_size;
        info!("start: {}", start);
        let res = proto::ds_tenant::ListDsTenantsResponse {
            total,
            page_size,
            total_list: items.into_iter().map(|v| v.into()).collect(),
            current_page: current_page + 1,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

    async fn get_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::GetDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenant>, tonic::Status> {
        todo!()
    }

    async fn create_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::CreateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenant>, tonic::Status> {
        let db = &self.db;
        let tenant_code = _req.get_ref().tenant_code.clone();
        let description = _req.get_ref().description.clone();
        let queue_id = _req.get_ref().queue_id;
        let res = ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            tenant_code: sea_orm::ActiveValue::Set(Some(tenant_code)),
            description: sea_orm::ActiveValue::Set(description),
            queue_id: sea_orm::ActiveValue::Set(queue_id),
            create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
            update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        }
        .insert(db)
        .await
        .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        Ok(tonic::Response::new(res.into()))
    }

    async fn update_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::UpdateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let db = &self.db;
        ActiveModel {
            id: sea_orm::ActiveValue::Set(_req.get_ref().id),
            tenant_code: sea_orm::ActiveValue::NotSet,
            description: sea_orm::ActiveValue::Set(Some(_req.get_ref().description.clone())),
            queue_id: sea_orm::ActiveValue::Set(Some(_req.get_ref().queue_id)),
            create_time: sea_orm::ActiveValue::NotSet,
            update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        }
        .update(db)
        .await
        .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        Ok(tonic::Response::new(()))
    }

    async fn delete_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::DeleteDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let db = &self.db;
        let id = _req.get_ref().id;
        Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        Ok(tonic::Response::new(()))
    }

    async fn verify_ds_tenant(
        &self,
        _req: tonic::Request<proto::ds_tenant::VerifyDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let tenant_code = _req.get_ref().tenant_code.clone();
        info!("tenant_code: {}", tenant_code);
        let res = Self::tenant_code_exist(self, &_req.get_ref().tenant_code).await?;

        if res {
            error!("tenant_code: {} exist", tenant_code);
            let err = Error::OsTenantCodeExist(AuroraData::Null, Some(vec![tenant_code.clone()]));
            error!("err: {:?}", err);
            let status = tonic::Status::from_error(Box::<AuroraErrorInfo>::new(err.into()));
            error!("status: {:?}", status);
            Err(status)
        } else {
            Ok(tonic::Response::new(()))
        }
    }
}
