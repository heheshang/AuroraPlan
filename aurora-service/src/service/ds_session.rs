use super::dao_service::AuroraRpcServer;
use aurora_common::{
    core_error::error::{AuroraErrorInfo, Error},
    core_results::results::{GrpcRequest, GrpcResponse},
};
use entity::t_ds_session::{self, Model};
use proto::ds_session::{
    ds_session_service_server::DsSessionService, DsSession, GetDsSessionByIdRequest,
    GetDsSessionUserIdRequest, GetDsSessionUserIdResponse,
};
use sea_orm::entity::prelude::*;

#[tonic::async_trait]
impl DsSessionService for AuroraRpcServer {
    async fn list_ds_sessions(
        &self,
        _req: tonic::Request<proto::ds_session::ListDsSessionsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_session::ListDsSessionsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_session(
        &self,
        _req: tonic::Request<proto::ds_session::GetDsSessionByIRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSession>, tonic::Status> {
        todo!()
    }

    async fn create_ds_session(
        &self,
        _req: tonic::Request<proto::ds_session::CreateDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSession>, tonic::Status> {
        let ds_session = _req.get_ref().ds_session.clone().unwrap();
        let id = ds_session.id.clone();
        let parse_from_str = DateTime::parse_from_str;
        let current_time = chrono::prelude::Local::now().naive_local();
        let model = t_ds_session::ActiveModel {
            id: sea_orm::ActiveValue::Set(id.clone()),
            user_id: sea_orm::ActiveValue::Set(Some(ds_session.user_id)),
            ip: sea_orm::ActiveValue::Set(ds_session.ip),
            last_login_time: sea_orm::ActiveValue::Set(Some(
                parse_from_str(&ds_session.last_login_time.unwrap(), "%Y-%m-%d %H:%M:%S")
                    .unwrap_or(current_time),
            )),
        };
        let conn = &self.conn;
        let _ = t_ds_session::Entity::insert(model.clone())
            .exec(conn)
            .await
            .map_err(|_| tonic::Status::internal("session  create failed"))?;
        let res: Option<Model> = t_ds_session::Entity::find()
            .filter(t_ds_session::Column::Id.eq(id.clone()))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("sessionId  not found"))?;
        match res {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::LoginSessionFailed.into(),
            ))),
        }
    }

    async fn update_ds_session(
        &self,
        req: tonic::Request<proto::ds_session::UpdateDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSession>, tonic::Status> {
        let ds_session = req.get_ref().ds_session.clone().unwrap();
        let id = ds_session.id.clone();
        let parse_from_str = DateTime::parse_from_str;
        let current_time = chrono::prelude::Local::now().naive_local();
        let model = t_ds_session::ActiveModel {
            id: sea_orm::ActiveValue::Set(ds_session.id),
            user_id: sea_orm::ActiveValue::Set(Some(ds_session.user_id)),
            ip: sea_orm::ActiveValue::Set(ds_session.ip),
            last_login_time: sea_orm::ActiveValue::Set(Some(
                parse_from_str(&ds_session.last_login_time.unwrap(), "%Y-%m-%d %H:%M:%S")
                    .unwrap_or(current_time),
            )),
        };

        let conn = &self.conn;
        let res = t_ds_session::Entity::update(model.clone())
            .filter(t_ds_session::Column::Id.eq(id))
            .exec(conn)
            .await
            .map_err(|_| tonic::Status::internal("session  update failed"))?;
        Ok(tonic::Response::new(res.into()))
    }

    async fn delete_ds_session(
        &self,
        request: tonic::Request<proto::ds_session::DeleteDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let conn = &self.conn;
        let id = request.into_inner().id;
        let del_res = t_ds_session::Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(|_| tonic::Status::not_found("sessionId  not found"))?;
        if del_res.rows_affected > 0 {
            Ok(tonic::Response::new(()))
        } else {
            Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::LoginSessionFailed.into(),
            )))
        }
    }

    async fn get_ds_session_by_id(
        &self,
        request: GrpcRequest<GetDsSessionByIdRequest>,
    ) -> GrpcResponse<DsSession> {
        let conn = &self.conn;
        let id = request.into_inner().id;
        let ds_session: Option<t_ds_session::Model> = t_ds_session::Entity::find()
            .filter(t_ds_session::Column::Id.eq(id))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("sessionId  not found"))?;
        match ds_session {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::LoginSessionFailed.into(),
            ))),
        }
    }

    async fn get_ds_session_by_user_id(
        &self,
        request: GrpcRequest<GetDsSessionUserIdRequest>,
    ) -> GrpcResponse<GetDsSessionUserIdResponse> {
        let conn = &self.conn;
        let user_id = request.into_inner().user_id;
        let ds_sessions: Vec<t_ds_session::Model> = t_ds_session::Entity::find()
            .filter(t_ds_session::Column::UserId.eq(user_id))
            .into_model()
            .all(conn)
            .await
            .map_err(|_| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::LoginSessionFailed.into(),
                ))
            })?;

        Ok(tonic::Response::new(GetDsSessionUserIdResponse {
            ds_sessions: ds_sessions.into_iter().map(|v| v.into()).collect(),
        }))
    }
}
