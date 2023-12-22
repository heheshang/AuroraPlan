use super::dao_service::AuroraRpcServer;
use lib_proto::ds_dq_execute_result::ds_dq_execute_result_service_server::DsDqExecuteResultService;

#[tonic::async_trait]
impl DsDqExecuteResultService for AuroraRpcServer {
    async fn list_ds_dq_execute_results(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_execute_result::ListDsDqExecuteResultsRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_dq_execute_result::ListDsDqExecuteResultsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_execute_result(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_execute_result::GetDsDqExecuteResultRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_execute_result::DsDqExecuteResult>, tonic::Status> {
        todo!()
    }

    async fn create_ds_dq_execute_result(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_execute_result::CreateDsDqExecuteResultRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_execute_result::DsDqExecuteResult>, tonic::Status> {
        todo!()
    }

    async fn update_ds_dq_execute_result(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_execute_result::UpdateDsDqExecuteResultRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_execute_result::DsDqExecuteResult>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_dq_execute_result(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_execute_result::DeleteDsDqExecuteResultRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
