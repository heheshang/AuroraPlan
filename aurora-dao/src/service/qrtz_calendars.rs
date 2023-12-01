use super::dao_service::AuroraRpcServer;
use proto::qrtz_calendars::qrtz_calendar_service_server::QrtzCalendarService;

#[tonic::async_trait]
impl QrtzCalendarService for AuroraRpcServer {
    async fn list_qrtz_calendars(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::ListQrtzCalendarsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::ListQrtzCalendarsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_qrtz_calendar(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::GetQrtzCalendarRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendar>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_calendar(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::CreateQrtzCalendarRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendar>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_calendar(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::UpdateQrtzCalendarRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendar>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_calendar(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::DeleteQrtzCalendarRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
