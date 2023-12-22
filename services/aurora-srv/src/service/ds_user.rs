use lib_models::t_ds_user::User;

use super::dao_service::AuroraRpcServer;
use lib_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::{GrpcRequest, GrpcResponse},
};
use lib_proto::ds_user::{
    ds_user_service_server::DsUserService, CreateDsUserRequest, DeleteDsUserRequest, DsUser, GetDsUserByIdRequest,
    GetDsUserByIdResponse, GetDsUserRequest, ListDsUsersRequest, ListDsUsersResponse, UpdateDsUserRequest,
};

#[tonic::async_trait]
impl DsUserService for AuroraRpcServer {
    async fn list_ds_users(&self, _req: GrpcRequest<ListDsUsersRequest>) -> GrpcResponse<ListDsUsersResponse> {
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        let (items, total_page, total, start, cur_page) = User::page(&search_val, page_num, page_size, pool)
            .await
            .map_err(|_| Error::InternalServerErrorArgs(AuroraData::Null, None))?;
        let res = ListDsUsersResponse {
            total,
            page_size,
            total_list: items.into_iter().map(|v| v.into()).collect(),
            current_page: cur_page,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

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
        req: tonic::Request<lib_proto::ds_user::QueryUserByNamePasswordRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_user::DsUser>, tonic::Status> {
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

    async fn create_ds_user(&self, _request: GrpcRequest<CreateDsUserRequest>) -> GrpcResponse<DsUser> {
        let pool = &self.pool;
        let user = _request.into_inner().ds_user.unwrap_or_default();
        User::create(
            user.user_name,
            user.user_password,
            user.email,
            user.phone,
            user.tenant_id,
            user.queue,
            user.state,
            pool,
        )
        .await
        .map(|v| tonic::Response::new(v.into()))
        .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn update_ds_user(&self, req: GrpcRequest<UpdateDsUserRequest>) -> GrpcResponse<DsUser> {
        let pool = &self.pool;
        if let Some(user) = req.into_inner().ds_user {
            User::update(
                user.id,
                user.user_name,
                user.tenant_id,
                user.email,
                user.queue,
                user.phone,
                user.state,
                pool,
            )
            .await
            .map_err(|_| Error::UserNotExist(AuroraData::Null, Some(vec![user.id.to_string()])))?;
            let db_user = User::find_by_id(user.id, pool)
                .await
                .map_err(|_| Error::UserNotExist(AuroraData::Null, Some(vec![user.id.to_string()])))?;
            Ok(tonic::Response::new(db_user.into()))
        } else {
            Err(Error::InternalServerErrorArgs(AuroraData::Null, None).into())
        }
    }

    async fn delete_ds_user(&self, _request: GrpcRequest<DeleteDsUserRequest>) -> GrpcResponse<()> {
        todo!()
    }

    async fn query_user_by_name(
        &self,
        request: tonic::Request<lib_proto::ds_user::QueryUserByNameRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_user::QueryUserByNameResponse>, tonic::Status> {
        let name = request.into_inner().user_name;
        let pool = &self.pool;
        User::query_user_by_name(&name, pool)
            .await
            .map(|v| {
                tonic::Response::new(lib_proto::ds_user::QueryUserByNameResponse {
                    user: v.map(|v| v.into()),
                })
            })
            .map_err(|_| Error::InternalServerErrorArgs(AuroraData::Null, None).into())
    }
}
