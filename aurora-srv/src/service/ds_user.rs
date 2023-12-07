use crate::models::t_ds_user::User;

use super::dao_service::AuroraRpcServer;
use aurora_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::{GrpcRequest, GrpcResponse},
};
use proto::ds_user::{
    ds_user_service_server::DsUserService, CreateDsUserRequest, DeleteDsUserRequest, DsUser, GetDsUserByIdRequest,
    GetDsUserByIdResponse, GetDsUserRequest, ListDsUsersRequest, ListDsUsersResponse, UpdateDsUserRequest,
};

#[tonic::async_trait]
impl DsUserService for AuroraRpcServer {
    async fn get_ds_user(&self, req: GrpcRequest<GetDsUserRequest>) -> GrpcResponse<DsUser> {
        let conn = &self.pool;
        let name = req.into_inner().name;
        let res = User::find_by_name(name.as_str(), conn)
            .await
            .map_err(|_| Error::UserNameExist(AuroraData::Null, Some(vec![name.clone()])))?;
        match res {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => {
                let res: tonic::Status = Error::UserNotExist(AuroraData::Null, Some(vec![name])).into();
                Err(res)
            }
        }
    }

    async fn update_ds_user(&self, req: GrpcRequest<UpdateDsUserRequest>) -> GrpcResponse<DsUser> {
        let conn = &self.pool;
        if let Some(user) = req.into_inner().ds_user {
            User::update(user.id, user.email, user.phone, user.user_type, conn)
                .await
                .map_err(|_| Error::UserNotExist(AuroraData::Null, Some(vec![user.id.to_string()])))?;
            let db_user = User::find_by_id(user.id, conn)
                .await
                .map_err(|_| Error::UserNotExist(AuroraData::Null, Some(vec![user.id.to_string()])))?;
            Ok(tonic::Response::new(db_user.into()))
        } else {
            Err(Error::InternalServerErrorArgs(AuroraData::Null, None).into())
        }
    }

    async fn list_ds_users(&self, _req: GrpcRequest<ListDsUsersRequest>) -> GrpcResponse<ListDsUsersResponse> {
        todo!()
    }

    async fn create_ds_user(&self, _request: GrpcRequest<CreateDsUserRequest>) -> GrpcResponse<DsUser> {
        todo!()
    }

    async fn delete_ds_user(&self, _request: GrpcRequest<DeleteDsUserRequest>) -> GrpcResponse<()> {
        todo!()
    }

    async fn get_ds_user_by_id(&self, req: GrpcRequest<GetDsUserByIdRequest>) -> GrpcResponse<GetDsUserByIdResponse> {
        let pool = &self.pool;
        let id = req.into_inner().id;
        let db_user = User::find_by_id(id, pool).await;
        match db_user {
            Ok(v) => Ok(tonic::Response::new(GetDsUserByIdResponse {
                ds_user: Some(v.into()),
            })),
            Err(_) => Ok(tonic::Response::new(GetDsUserByIdResponse { ds_user: None })),
        }
    }

    async fn query_user_by_name_password(
        &self,
        req: tonic::Request<proto::ds_user::QueryUserByNamePasswordRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_user::DsUser>, tonic::Status> {
        let pool = &self.pool;
        let user_name = &req.get_ref().user_name;
        let user_password = &req.get_ref().user_password;
        User::query_user_by_name_password(user_name, user_password, pool)
            .await
            .map(|v| {
                tonic::Response::new(
                    match v {
                        Some(x) => x,
                        None => User::default(),
                    }
                    .into(),
                )
            })
            .map_err(|_| Error::UserNotExist(AuroraData::Null, Some(vec![user_name.to_string()])).into())
    }
}
