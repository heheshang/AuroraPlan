use super::dao_service::AuroraRpcServer;
use lib_proto::qrtz_fired_triggers::qrtz_fired_triggers_service_server::QrtzFiredTriggersService;

#[tonic::async_trait]
impl QrtzFiredTriggersService for AuroraRpcServer {
    async fn list_qrtz_fired_triggerss(
        &self,
        _req: tonic::Request<lib_proto::qrtz_fired_triggers::ListQrtzFiredTriggerssRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::qrtz_fired_triggers::ListQrtzFiredTriggerssResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_fired_triggers(
        &self,
        _req: tonic::Request<lib_proto::qrtz_fired_triggers::GetQrtzFiredTriggersRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_fired_triggers::QrtzFiredTriggers>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_fired_triggers(
        &self,
        _req: tonic::Request<lib_proto::qrtz_fired_triggers::CreateQrtzFiredTriggersRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_fired_triggers::QrtzFiredTriggers>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_fired_triggers(
        &self,
        _req: tonic::Request<lib_proto::qrtz_fired_triggers::UpdateQrtzFiredTriggersRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_fired_triggers::QrtzFiredTriggers>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_fired_triggers(
        &self,
        _req: tonic::Request<lib_proto::qrtz_fired_triggers::DeleteQrtzFiredTriggersRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
