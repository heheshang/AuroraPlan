use super::dao_service::AuroraRpcServer;
use proto::ds_project::ds_project_service_server::DsProjectService;

#[tonic::async_trait]
impl DsProjectService for AuroraRpcServer {
    async fn list_ds_projects(
        &self,
        _req: tonic::Request<proto::ds_project::ListDsProjectsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project::ListDsProjectsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::GetDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        todo!()
    }

    async fn create_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::CreateDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        todo!()
    }

    async fn update_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::UpdateDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProject>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_project(
        &self,
        _req: tonic::Request<proto::ds_project::DeleteDsProjectRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
