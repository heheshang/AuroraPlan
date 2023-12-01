use super::dao_service::AuroraRpcServer;

use proto::qrtz_triggers::qrtz_trigger_service_server::QrtzTriggerService;
#[tonic::async_trait]
impl QrtzTriggerService for AuroraRpcServer {
    async fn list_qrtz_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::ListQrtzTriggersRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::ListQrtzTriggersResponse>, tonic::Status> {
        todo!()
    }

    async fn get_qrtz_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::GetQrtzTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTrigger>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::CreateQrtzTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTrigger>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::UpdateQrtzTriggerRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTrigger>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::DeleteQrtzTriggerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
