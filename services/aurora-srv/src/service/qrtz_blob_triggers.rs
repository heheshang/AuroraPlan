use super::dao_service::AuroraRpcServer;
use lib_proto::qrtz_blob_triggers::qrtz_blob_trigger_service_server::QrtzBlobTriggerService;

#[tonic::async_trait]
impl QrtzBlobTriggerService for AuroraRpcServer {
    async fn list_qrtz_blob_triggers(
        &self,
        _req: tonic::Request<lib_proto::qrtz_blob_triggers::ListQrtzBlobTriggersRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_blob_triggers::ListQrtzBlobTriggersResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_qrtz_blob_trigger(
        &self,
        _req: tonic::Request<lib_proto::qrtz_blob_triggers::GetQrtzBlobTriggerRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_blob_triggers::QrtzBlobTrigger>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_blob_trigger(
        &self,
        _req: tonic::Request<lib_proto::qrtz_blob_triggers::CreateQrtzBlobTriggerRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_blob_triggers::QrtzBlobTrigger>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_blob_trigger(
        &self,
        _req: tonic::Request<lib_proto::qrtz_blob_triggers::UpdateQrtzBlobTriggerRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::qrtz_blob_triggers::QrtzBlobTrigger>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_blob_trigger(
        &self,
        _req: tonic::Request<lib_proto::qrtz_blob_triggers::DeleteQrtzBlobTriggerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
