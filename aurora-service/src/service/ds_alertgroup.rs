use super::dao_service::AuroraRpcServer;
use proto::ds_alertgroup::ds_alert_group_service_server::DsAlertGroupService;

#[tonic::async_trait]
impl DsAlertGroupService for AuroraRpcServer {
    async fn list_ds_alert_groups(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::ListDsAlertGroupsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alertgroup::ListDsAlertGroupsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::GetDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::CreateDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::UpdateDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroup>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_alert_group(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::DeleteDsAlertGroupRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
