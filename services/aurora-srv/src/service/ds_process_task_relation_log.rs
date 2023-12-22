use super::dao_service::AuroraRpcServer;
use lib_proto::ds_process_task_relation_log::ds_process_task_relation_log_service_server::DsProcessTaskRelationLogService;

#[tonic::async_trait]
impl DsProcessTaskRelationLogService for AuroraRpcServer {
    async fn list_ds_process_task_relation_logs(
        &self,
        _req: tonic::Request<lib_proto::ds_process_task_relation_log::ListDsProcessTaskRelationLogsRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_process_task_relation_log::ListDsProcessTaskRelationLogsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_task_relation_log(
        &self,
        _req: tonic::Request<lib_proto::ds_process_task_relation_log::GetDsProcessTaskRelationLogRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_process_task_relation_log::DsProcessTaskRelationLog>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_task_relation_log(
        &self,
        _req: tonic::Request<lib_proto::ds_process_task_relation_log::CreateDsProcessTaskRelationLogRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_process_task_relation_log::DsProcessTaskRelationLog>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_task_relation_log(
        &self,
        _req: tonic::Request<lib_proto::ds_process_task_relation_log::UpdateDsProcessTaskRelationLogRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_process_task_relation_log::DsProcessTaskRelationLog>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_task_relation_log(
        &self,
        _req: tonic::Request<lib_proto::ds_process_task_relation_log::DeleteDsProcessTaskRelationLogRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
