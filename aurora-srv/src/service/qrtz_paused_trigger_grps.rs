use super::dao_service::AuroraRpcServer;
use proto::qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_service_server::QrtzPausedTriggerGrpsService;

#[tonic::async_trait]
impl QrtzPausedTriggerGrpsService for AuroraRpcServer {
    async fn list_qrtz_paused_trigger_grpss(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::ListQrtzPausedTriggerGrpssRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_paused_trigger_grps::ListQrtzPausedTriggerGrpssResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_paused_trigger_grps(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::GetQrtzPausedTriggerGrpsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrps>, tonic::Status>
    {
        todo!()
    }

    async fn create_qrtz_paused_trigger_grps(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::CreateQrtzPausedTriggerGrpsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrps>, tonic::Status>
    {
        todo!()
    }

    async fn update_qrtz_paused_trigger_grps(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::UpdateQrtzPausedTriggerGrpsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrps>, tonic::Status>
    {
        todo!()
    }

    async fn delete_qrtz_paused_trigger_grps(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::DeleteQrtzPausedTriggerGrpsRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
