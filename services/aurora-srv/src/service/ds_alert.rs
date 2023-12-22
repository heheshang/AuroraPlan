use super::dao_service::AuroraRpcServer;
use lib_proto::ds_alert::ds_alert_service_server::DsAlertService;
#[tonic::async_trait]
impl DsAlertService for AuroraRpcServer {
    async fn list_ds_alerts(
        &self,
        _req: tonic::Request<lib_proto::ds_alert::ListDsAlertsRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert::ListDsAlertsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_alert(
        &self,
        _req: tonic::Request<lib_proto::ds_alert::GetDsAlertRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert::DsAlert>, tonic::Status> {
        todo!()
    }

    async fn create_ds_alert(
        &self,
        _req: tonic::Request<lib_proto::ds_alert::CreateDsAlertRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert::DsAlert>, tonic::Status> {
        todo!()
    }

    async fn update_ds_alert(
        &self,
        _req: tonic::Request<lib_proto::ds_alert::UpdateDsAlertRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_alert::DsAlert>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_alert(
        &self,
        _req: tonic::Request<lib_proto::ds_alert::DeleteDsAlertRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
