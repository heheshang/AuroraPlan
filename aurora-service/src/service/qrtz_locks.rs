use super::dao_service::AuroraRpcServer;
use proto::qrtz_locks::qrtz_locks_service_server::QrtzLocksService;

#[tonic::async_trait]
impl QrtzLocksService for AuroraRpcServer {
    async fn list_qrtz_lockss(
        &self,
        _req: tonic::Request<proto::qrtz_locks::ListQrtzLockssRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_locks::ListQrtzLockssResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_locks(
        &self,
        _req: tonic::Request<proto::qrtz_locks::GetQrtzLocksRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocks>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_locks(
        &self,
        _req: tonic::Request<proto::qrtz_locks::CreateQrtzLocksRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocks>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_locks(
        &self,
        _req: tonic::Request<proto::qrtz_locks::UpdateQrtzLocksRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocks>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_locks(
        &self,
        _req: tonic::Request<proto::qrtz_locks::DeleteQrtzLocksRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
