use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_session::Model;
use lib_common::{
    core_error::error::{AuroraData, AuroraErrorInfo, Error},
    core_results::results::{GrpcRequest, GrpcResponse},
};
use lib_proto::ds_session::{
    ds_session_service_server::DsSessionService, DsSession, GetDsSessionByIdRequest, GetDsSessionUserIdRequest,
    GetDsSessionUserIdResponse,
};
#[tonic::async_trait]
impl DsSessionService for AuroraRpcServer {
    async fn list_ds_sessions(
        &self,
        _req: tonic::Request<lib_proto::ds_session::ListDsSessionsRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_session::ListDsSessionsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_session(
        &self,
        _req: tonic::Request<lib_proto::ds_session::GetDsSessionByIRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_session::DsSession>, tonic::Status> {
        todo!()
    }

    async fn create_ds_session(
        &self,
        _req: tonic::Request<lib_proto::ds_session::CreateDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_session::DsSession>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().ds_session.clone().unwrap().id;
        let user_id = _req.get_ref().ds_session.clone().unwrap().user_id;
        let ip = _req.get_ref().ds_session.clone().unwrap().ip.clone();
        Model::create(id, user_id, ip, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::LoginSessionFailed(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn update_ds_session(
        &self,
        req: tonic::Request<lib_proto::ds_session::UpdateDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_session::DsSession>, tonic::Status> {
        let pool = &self.pool;
        let id = req.get_ref().ds_session.clone().unwrap().id;
        let user_id = req.get_ref().ds_session.clone().unwrap().user_id;
        let ip = req.get_ref().ds_session.clone().unwrap().ip.clone();
        Model::update(id, user_id, ip, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::LoginSessionFailed(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn delete_ds_session(
        &self,
        request: tonic::Request<lib_proto::ds_session::DeleteDsSessionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let id = request.into_inner().id;
        Model::delete(id, pool).await.map(|_| tonic::Response::new(())).map_err(|_| {
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })
    }

    async fn get_ds_session_by_id(&self, request: GrpcRequest<GetDsSessionByIdRequest>) -> GrpcResponse<DsSession> {
        let id = request.into_inner().id;
        let pool = &self.pool;
        Model::find_by_id(id, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn get_ds_session_by_user_id(
        &self,
        request: GrpcRequest<GetDsSessionUserIdRequest>,
    ) -> GrpcResponse<GetDsSessionUserIdResponse> {
        let user_id = request.into_inner().user_id;
        let pool = &self.pool;
        Model::find_by_user_id(user_id, pool)
            .await
            .map(|v| {
                let res = v.into_iter().map(|v| v.into()).collect::<Vec<DsSession>>();

                tonic::Response::new(GetDsSessionUserIdResponse { ds_sessions: res })
            })
            .map_err(|_| {
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }
}
