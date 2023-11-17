use super::dao_service::AuroraRpcServer;
use aurora_common::{
    core_error::error::{AuroraData, AuroraErrorInfo, Error},
    core_results::results::{GrpcRequest, GrpcResponse},
};
use entity::t_ds_user::{self};
use proto::ds_user::{
    ds_user_service_server::DsUserService, CreateDsUserRequest, DeleteDsUserRequest, DsUser, GetDsUserByIdRequest,
    GetDsUserByIdResponse, GetDsUserRequest, ListDsUsersRequest, ListDsUsersResponse, UpdateDsUserRequest,
};
use sea_orm::entity::prelude::*;

#[tonic::async_trait]
impl DsUserService for AuroraRpcServer {
    async fn get_ds_user(&self, req: GrpcRequest<GetDsUserRequest>) -> GrpcResponse<DsUser> {
        let conn = &self.db;
        let name = req.into_inner().name;
        let db_user: Option<t_ds_user::Model> = t_ds_user::Entity::find()
            .filter(t_ds_user::Column::UserName.eq(name))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("User not found"))?;
        match db_user {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => {
                let res: tonic::Status = Error::UserNotExist(AuroraData::Null, None).into();
                Err(res)
            }
        }
    }

    async fn update_ds_user(&self, req: GrpcRequest<UpdateDsUserRequest>) -> GrpcResponse<DsUser> {
        let conn = &self.db;
        if let Some(user) = req.into_inner().ds_user {
            let db_user = t_ds_user::Entity::find_by_id(user.id)
                .one(conn)
                .await
                .map_err(|_| tonic::Status::not_found("User not found"))?;
            let mut db_user = db_user.unwrap();

            db_user.email = user.email;
            db_user.phone = user.phone;
            db_user.user_type = user.user_type;
            Ok(tonic::Response::new(db_user.into()))
        } else {
            Err(tonic::Status::not_found("User not found"))
            // Ok(tonic::Response::new(DsUser::default()))
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
        let conn = &self.db;
        let id = req.into_inner().id;
        let db_user: Option<t_ds_user::Model> = t_ds_user::Entity::find()
            .filter(t_ds_user::Column::Id.eq(id))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("User not found"))?;
        match db_user {
            Some(v) => Ok(tonic::Response::new(GetDsUserByIdResponse {
                ds_user: Some(v.into()),
            })),
            None => Ok(tonic::Response::new(GetDsUserByIdResponse { ds_user: None })),
        }
    }

    async fn query_user_by_name_password(
        &self,
        req: tonic::Request<proto::ds_user::QueryUserByNamePasswordRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_user::DsUser>, tonic::Status> {
        let conn = &self.db;
        let user_name = &req.get_ref().user_name;
        let user_password = &req.get_ref().user_password;
        let db_user: Option<t_ds_user::Model> = t_ds_user::Entity::find()
            .filter(t_ds_user::Column::UserName.eq(user_name))
            .filter(t_ds_user::Column::UserPassword.eq(user_password))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("User not found"))?;
        match db_user {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::UserNotExist(AuroraData::Null, None).into(),
            ))),
        }
    }
}
