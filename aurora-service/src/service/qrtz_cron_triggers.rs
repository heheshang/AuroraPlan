use super::dao_service::AuroraRpcServer;
use proto::qrtz_cron_triggers::qrtz_cron_triggers_service_server::QrtzCronTriggersService;

#[tonic::async_trait]
impl QrtzCronTriggersService for AuroraRpcServer {
    async fn list_qrtz_cron_triggerss(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::ListQrtzCronTriggerssRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::ListQrtzCronTriggerssResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_cron_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::GetQrtzCronTriggersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggers>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_cron_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::CreateQrtzCronTriggersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggers>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_cron_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::UpdateQrtzCronTriggersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggers>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_cron_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::DeleteQrtzCronTriggersRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
