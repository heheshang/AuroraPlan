use super::dao_service::AuroraRpcServer;
use lib_common::core_error::error::AuroraData;
use lib_common::core_error::error::AuroraErrorInfo;
use lib_common::core_error::error::Error;
use lib_models::t_ds_tenant::Model;
use lib_proto::ds_tenant::ds_tenant_service_server::DsTenantService;
use tracing::error;
use tracing::info;

#[tonic::async_trait]
impl DsTenantService for AuroraRpcServer {
    async fn list_ds_tenants(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::ListDsTenantsRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_tenant::ListDsTenantsResponse>, tonic::Status> {
        let pool = &self.pool;
        let search_val = _req.get_ref().clone().search_val.unwrap_or_default();
        let page_size = _req.get_ref().clone().page_size;
        let page_num = _req.get_ref().clone().page_num;
        info!(
            "search_val: {},page_size: {} ,page_num:{}",
            search_val, page_size, page_num
        );
        let (items, total_page, total, start, cur_page) = Model::page(&search_val, page_num, page_size, pool)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;

        let res = lib_proto::ds_tenant::ListDsTenantsResponse {
            total,
            page_size,
            total_list: items.into_iter().map(|v| v.into()).collect(),
            current_page: cur_page,
            start,
            total_page,
        };
        Ok(tonic::Response::new(res))
    }

    async fn all_ds_tenants(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_tenant::AllDsTenantsResponse>, tonic::Status> {
        let pool = &self.pool;
        Model::all(pool)
            .await
            .map(|v| {
                tonic::Response::new(lib_proto::ds_tenant::AllDsTenantsResponse {
                    total_list: v.into_iter().map(|v| v.into()).collect(),
                })
            })
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn get_ds_tenant(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::GetDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_tenant::DsTenant>, tonic::Status> {
        todo!()
    }

    async fn create_ds_tenant(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::CreateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_tenant::DsTenant>, tonic::Status> {
        let pool = &self.pool;
        let tenant_code = _req.get_ref().tenant_code.clone();
        let description = _req.get_ref().description.clone();
        let queue_id = _req.get_ref().queue_id;
        Model::create(Some(tenant_code), description, queue_id, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn update_ds_tenant(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::UpdateDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        let description = _req.get_ref().description.clone();
        let queue_id = _req.get_ref().queue_id;
        Model::update(id, description, queue_id, pool)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn delete_ds_tenant(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::DeleteDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let id = _req.get_ref().id;
        Model::delete(id, pool)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))
    }

    async fn verify_ds_tenant(
        &self,
        _req: tonic::Request<lib_proto::ds_tenant::VerifyDsTenantRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;

        let tenant_code = _req.get_ref().tenant_code.clone();
        let res = Model::find_by_code(&tenant_code, pool)
            .await
            .map_err(|_| tonic::Status::from_error(Error::InternalServerErrorArgs(AuroraData::Null, None).into()))?;
        match res {
            Some(_) => {
                let err = Error::OsTenantCodeExist(AuroraData::Null, Some(vec![tenant_code.clone()]));
                error!("err: {:?}", err);
                let status = tonic::Status::from_error(Box::<AuroraErrorInfo>::new(err.into()));
                // let status = tonic::Status::from_error(err.into());
                error!("status: {:?}", status);
                Err(status)
            }
            None => Ok(tonic::Response::new(())),
        }
    }
}
