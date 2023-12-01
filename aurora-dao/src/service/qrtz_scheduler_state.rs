use super::dao_service::AuroraRpcServer;
use proto::qrtz_scheduler_state::qrtz_scheduler_state_service_server::QrtzSchedulerStateService;

#[tonic::async_trait]
impl QrtzSchedulerStateService for AuroraRpcServer {
    async fn list_qrtz_scheduler_states(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::ListQrtzSchedulerStatesRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_scheduler_state::ListQrtzSchedulerStatesResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_qrtz_scheduler_state(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::GetQrtzSchedulerStateRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerState>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_scheduler_state(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::CreateQrtzSchedulerStateRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerState>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_scheduler_state(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::UpdateQrtzSchedulerStateRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerState>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_scheduler_state(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::DeleteQrtzSchedulerStateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
