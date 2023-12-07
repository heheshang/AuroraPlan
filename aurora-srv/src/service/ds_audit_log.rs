use super::dao_service::AuroraRpcServer;
use proto::ds_audit_log::ds_audit_log_service_server::DsAuditLogService;

#[tonic::async_trait]
impl DsAuditLogService for AuroraRpcServer {
    async fn list_ds_audit_logs(
        &self,
        _req: tonic::Request<proto::ds_audit_log::ListDsAuditLogsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::ListDsAuditLogsResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_audit_log(
        &self,
        _req: tonic::Request<proto::ds_audit_log::GetDsAuditLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLog>, tonic::Status> {
        todo!()
    }

    async fn create_ds_audit_log(
        &self,
        _req: tonic::Request<proto::ds_audit_log::CreateDsAuditLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLog>, tonic::Status> {
        todo!()
    }

    async fn update_ds_audit_log(
        &self,
        _req: tonic::Request<proto::ds_audit_log::UpdateDsAuditLogRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLog>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_audit_log(
        &self,
        _req: tonic::Request<proto::ds_audit_log::DeleteDsAuditLogRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
