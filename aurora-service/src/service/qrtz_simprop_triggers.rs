use super::dao_service::AuroraRpcServer;
use proto::qrtz_simprop_triggers::qrtz_simprop_trigger_service_server::QrtzSimpropTriggerService;

#[tonic::async_trait]
impl QrtzSimpropTriggerService for AuroraRpcServer {
    async fn list_qrtz_simprop_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::ListQrtzSimpropTriggersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simprop_triggers::ListQrtzSimpropTriggersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_simprop_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::GetQrtzSimpropTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTrigger>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_simprop_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::CreateQrtzSimpropTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTrigger>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_simprop_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::UpdateQrtzSimpropTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTrigger>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_simprop_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::DeleteQrtzSimpropTriggerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
