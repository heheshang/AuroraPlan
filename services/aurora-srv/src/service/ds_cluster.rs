use super::dao_service::AuroraRpcServer;
use crate::models::t_ds_cluster::Model;
use lib_common::core_error::error::{AuroraData, AuroraErrorInfo, Error};
use lib_proto::ds_cluster::ds_cluster_service_server::DsClusterService;
use tracing::error;

#[tonic::async_trait]
impl DsClusterService for AuroraRpcServer {
    async fn list_ds_clusters(
        &self,
        _req: tonic::Request<lib_proto::ds_cluster::ListDsClustersRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_cluster::ListDsClustersResponse>, tonic::Status> {
        let pool = &self.pool;
        let page_size = _req.get_ref().page_size;
        let page_num = _req.get_ref().page_num;
        let search_val = _req.get_ref().search_val.clone().unwrap_or_default();
        Model::page(&search_val, page_num, page_size, pool)
            .await
            .map(|(items, total_page, total, start, current_page)| {
                tonic::Response::new(lib_proto::ds_cluster::ListDsClustersResponse {
                    total_list: items.into_iter().map(|instance| instance.into()).collect(),
                    total_page,
                    total,
                    start,
                    current_page,
                    page_size,
                })
            })
            .map_err(|_e| {
                error!("list_ds_clusters error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn get_ds_cluster(
        &self,
        _req: tonic::Request<lib_proto::ds_cluster::GetDsClusterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_cluster::DsCluster>, tonic::Status> {
        todo!()
    }

    async fn create_ds_cluster(
        &self,
        _req: tonic::Request<lib_proto::ds_cluster::CreateDsClusterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_cluster::DsCluster>, tonic::Status> {
        let pool = &self.pool;
        let name: Option<String> = _req.get_ref().name.clone();
        let config: Option<String> = _req.get_ref().config.clone();
        let description: Option<String> = _req.get_ref().description.clone();
        let operator: Option<i32> = _req.get_ref().operator;
        Model::_create(name, config, description, operator, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("create_ds_cluster error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn update_ds_cluster(
        &self,
        _req: tonic::Request<lib_proto::ds_cluster::UpdateDsClusterRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_cluster::DsCluster>, tonic::Status> {
        let pool = &self.pool;
        let code = _req.get_ref().code;
        let name: Option<String> = _req.get_ref().name.clone();
        let config: Option<String> = _req.get_ref().config.clone();
        let description: Option<String> = _req.get_ref().description.clone();
        let operator: Option<i32> = _req.get_ref().operator;
        Model::_update(code, name, config, description, operator, pool)
            .await
            .map(|v| tonic::Response::new(v.into()))
            .map_err(|e| {
                error!("update_ds_cluster error: {:?}", e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn delete_ds_cluster(
        &self,
        _req: tonic::Request<lib_proto::ds_cluster::DeleteDsClusterRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let code = _req.get_ref().code;
        Model::_delete(code, pool)
            .await
            .map(|_| tonic::Response::new(()))
            .map_err(|_e| {
                error!("delete_ds_cluster error: {:?}", _e);
                tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                    Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
                ))
            })
    }

    async fn verify_ds_cluster(
        &self,
        _request: tonic::Request<lib_proto::ds_cluster::VerifyClusterRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let pool = &self.pool;
        let name = _request.get_ref().name.clone();
        let res = Model::_find_by_name(name.clone(), pool).await.map_err(|e| {
            error!("verify_cluster error: {:?}", e);
            tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::InternalServerErrorArgs(AuroraData::Null, None).into(),
            ))
        })?;

        match res {
            None => Ok(tonic::Response::new(())),
            Some(_) => Err(tonic::Status::from_error(Box::<AuroraErrorInfo>::new(
                Error::ClusterNameExists(AuroraData::Null, Some(vec![name])).into(),
            ))),
        }
    }
}
