use aurora_common::{
    core_error::error::Error,
    core_results::results::{GrpcRequest, GrpcResponse},
};
use entity::t_ds_access_token;
use proto::ds_access_token::{
    ds_access_token_service_server::DsAccessTokenService, CreateDsAccessTokenRequest,
    DeleteDsAccessTokenRequest, DsAccessToken, GetDsAccessTokenRequest, ListDsAccessTokensRequest,
    ListDsAccessTokensResponse, UpdateDsAccessTokenRequest,
};
use sea_orm::{entity::prelude::*, QueryOrder};

use super::dao_service::AuroraRpcServer;

#[tonic::async_trait]
impl DsAccessTokenService for AuroraRpcServer {
    async fn list_ds_access_tokens(
        &self,
        req: GrpcRequest<ListDsAccessTokensRequest>,
    ) -> GrpcResponse<ListDsAccessTokensResponse> {
        let page_size = req.get_ref().page_size;
        let page_num = req.get_ref().page_num;

        let paginator = t_ds_access_token::Entity::find()
            .order_by_asc(t_ds_access_token::Column::Id)
            .paginate(&self.conn, page_size);

        let num_pages = paginator
            .num_pages()
            .await
            .map_err(|_| Into::<tonic::Status>::into(Error::InternalServerErrorArgs))?
            as i32;
        // Fetch paginated AccessToken
        let res: (Vec<t_ds_access_token::Model>, i32) = paginator
            .fetch_page(page_num - 1)
            .await
            .map(|p| (p, num_pages))
            .map_err(|_| {
                let res: tonic::Status = Error::InternalServerErrorArgs.into();
                res
            })?;

        let ss: Vec<DsAccessToken> = res.0.into_iter().map(|v| v.into()).collect();
        Ok(tonic::Response::new(ListDsAccessTokensResponse {
            ds_access_tokens: ss,
        }))
    }

    async fn get_ds_access_token(
        &self,
        _req: GrpcRequest<GetDsAccessTokenRequest>,
    ) -> GrpcResponse<DsAccessToken> {
        todo!()
    }

    async fn create_ds_access_token(
        &self,
        _req: GrpcRequest<CreateDsAccessTokenRequest>,
    ) -> GrpcResponse<DsAccessToken> {
        todo!()
    }

    async fn update_ds_access_token(
        &self,
        _req: GrpcRequest<UpdateDsAccessTokenRequest>,
    ) -> GrpcResponse<DsAccessToken> {
        todo!()
    }

    async fn delete_ds_access_token(
        &self,
        _req: GrpcRequest<DeleteDsAccessTokenRequest>,
    ) -> GrpcResponse<()> {
        todo!()
    }
}
