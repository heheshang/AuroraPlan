use super::dao_service::AuroraRpcServer;
use aurora_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use entity::t_ds_project_parameter::{self, ActiveModel, Column, Entity};
use proto::ds_project_parameter::project_parameter_service_server::ProjectParameterService;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, QueryOrder, Set};
use snowflake::SnowflakeIdBucket;
use tracing::{error, info};

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
        let res = ActiveModel {
            id: NotSet,
            code: Set(SnowflakeIdBucket::new(1, 1).get_id()),
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
                Error::InternalServerErrorArgs(AuroraData::Null).into(),
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
        todo!()
    }

    async fn delete_project_parameter(
        &self,
        _request: tonic::Request<proto::ds_project_parameter::DeleteProjectParameterRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        info!("request: {:?}", _request);
        todo!()
    }
}
