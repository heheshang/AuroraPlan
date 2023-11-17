use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use entity::t_ds_project_parameter::{self, ActiveModel, Column, Entity, Model};
use proto::ds_project_parameter::project_parameter_service_server::ProjectParameterService;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, QueryOrder, Set};
use sea_orm::{DeleteResult, TransactionTrait};
use tracing::{error, info};
type Result<T> = std::result::Result<T, tonic::Status>;
impl AuroraRpcServer {
    async fn find_by_code_projectcode(
        &self,
        code: i64,
        project_code: i64,
    ) -> Result<Option<Model>> {
        let conn: &DatabaseConnection = &self.conn;
        let res = Entity::find()
            .filter(t_ds_project_parameter::Column::Code.eq(code))
            .filter(t_ds_project_parameter::Column::ProjectCode.eq(project_code))
            .one(conn)
            .await
            .map_err(|_| {
                tonic::Status::from_error(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                )
            })?;
        Ok(res)
    }
}

#[tonic::async_trait]
impl ProjectParameterService for AuroraRpcServer {
    async fn list_project_parameters(
        &self,
        _req: tonic::Request<proto::ds_project_parameter::ListProjectParametersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project_parameter::ListProjectParametersResponse>,
        tonic::Status,
    > {
        info!("request: {:?}", _req);
        let conn = &self.conn;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        let project_code = _req.get_ref().clone().project_code;
        info!(
            "search_val: {},page_size: {} ,page_num:{} ,project_code:{}",
            search_val, page_size, page_num, project_code
        );
        let pages = Entity::find()
            .order_by_desc(Column::CreateTime)
            .filter(t_ds_project_parameter::Column::ParamName.like(format!("%{}%", search_val)))
            .filter(t_ds_project_parameter::Column::ProjectCode.eq(project_code as i64))
            .paginate(conn, page_size);
        info!("query sql:{:#?}", pages);
        let page_num = match page_num {
            0 => 0,
            _ => page_num - 1,
        };
        let items = match pages.fetch_page(page_num).await {
            Ok(items) => items,
            Err(_) => {
                error!("fetch_page ds_project_parameter error");
                vec![]
            }
        };
        let current_page = pages.cur_page();
        info!("current_page: {}", current_page);
        let (total, total_page) = match pages.num_items_and_pages().await {
            Ok(v) => (v.number_of_items, v.number_of_pages),
            Err(_) => {
                error!("num_items_and_pages ds_project_parameter error");
                (0, 0)
            }
        };
        info!("total: {}, total_page: {}", total, total_page);
        let start = (page_num) * page_size;
        info!("start: {}", start);
        let res = proto::ds_project_parameter::ListProjectParametersResponse {
            total,
            page_size,
            total_list: items.into_iter().map(|v| v.into()).collect(),
            current_page: current_page + 1,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

    async fn get_project_parameter(
        &self,
        _request: tonic::Request<proto::ds_project_parameter::GetProjectParameterRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project_parameter::ProjectParameter>,
        tonic::Status,
    > {
        info!("request: {:?}", _request);
        todo!()
    }

    async fn create_project_parameter(
        &self,
        _request: tonic::Request<proto::ds_project_parameter::CreateProjectParameterRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project_parameter::ProjectParameter>,
        tonic::Status,
    > {
        info!("request: {:?}", _request);
        let conn = &self.conn;
        let project_parameter = _request.get_ref().project_parameter.clone().unwrap();
        let code = aurora_common::utils::code_generate_utils::gen_code().unwrap_or_default();
        let res = ActiveModel {
            id: NotSet,
            code: Set(code),
            user_id: Set(project_parameter.user_id),
            project_code: Set(project_parameter.project_code),
            param_name: Set(project_parameter.param_name),
            param_value: Set(project_parameter.param_value),
            create_time: Set(Some(chrono::prelude::Local::now().naive_local())),
            update_time: Set(Some(chrono::prelude::Local::now().naive_local())),
        }
        .insert(conn)
        .await;
        match res {
            Ok(v) => Ok(tonic::Response::new(v.into())),
            Err(_) => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))),
        }
    }

    async fn update_project_parameter(
        &self,
        _request: tonic::Request<proto::ds_project_parameter::UpdateProjectParameterRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project_parameter::ProjectParameter>,
        tonic::Status,
    > {
        info!("request: {:?}", _request);
        let conn = &self.conn;
        let code = _request.get_ref().code;
        let project_code = _request.get_ref().project_code;
        let param_name = _request.get_ref().param_name.clone();
        let param_value = _request.get_ref().param_value.clone();
        Entity::update_many()
            .col_expr(Column::ParamName, Expr::value(param_name))
            .col_expr(Column::ParamValue, Expr::value(param_value))
            .filter(t_ds_project_parameter::Column::Code.eq(code))
            .filter(t_ds_project_parameter::Column::ProjectCode.eq(project_code))
            .exec(conn)
            .await
            .map_err(|e| {
                error!("update_project_parameter error {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?;

        let res = Self::find_by_code_projectcode(self, code, project_code).await?;
        info!("update_project_parameter res: {:?}", res);
        match res {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))),
        }
    }

    async fn delete_project_parameter(
        &self,
        _request: tonic::Request<proto::ds_project_parameter::DeleteProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        info!("request: {:?}", _request);
        let conn = &self.conn;
        let code = _request.get_ref().code;
        let project_code = _request.get_ref().project_code;
        let res = conn
            .transaction::<_, DeleteResult, DbErr>(|tx| {
                Box::pin(async move {
                    let res = Entity::delete_many()
                        .filter(t_ds_project_parameter::Column::Code.eq(code))
                        .filter(t_ds_project_parameter::Column::ProjectCode.eq(project_code))
                        .exec(tx)
                        .await?;
                    Ok(res)
                })
            })
            .await
            .map_err(|_e| {
                error!("delete_project_parameter error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })?;
        info!("delete_project_parameter res: {:?}", res);
        Ok(tonic::Response::new(()))
    }
}
