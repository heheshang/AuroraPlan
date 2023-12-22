use super::dao_service::AuroraRpcServer;
use lib_proto::ds_schedules::ds_schedules_service_server::DsSchedulesService;

#[tonic::async_trait]
impl DsSchedulesService for AuroraRpcServer {
    async fn list_ds_scheduless(
        &self,
        _req: tonic::Request<lib_proto::ds_schedules::ListDsSchedulessRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_schedules::ListDsSchedulessResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_schedules(
        &self,
        _req: tonic::Request<lib_proto::ds_schedules::GetDsSchedulesRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_schedules::DsSchedules>, tonic::Status> {
        todo!()
    }

    async fn create_ds_schedules(
        &self,
        _req: tonic::Request<lib_proto::ds_schedules::CreateDsSchedulesRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_schedules::DsSchedules>, tonic::Status> {
        todo!()
    }

    async fn update_ds_schedules(
        &self,
        _req: tonic::Request<lib_proto::ds_schedules::UpdateDsSchedulesRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_schedules::DsSchedules>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_schedules(
        &self,
        _req: tonic::Request<lib_proto::ds_schedules::DeleteDsSchedulesRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
