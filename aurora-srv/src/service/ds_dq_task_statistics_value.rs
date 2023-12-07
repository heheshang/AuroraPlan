use super::dao_service::AuroraRpcServer;
use proto::ds_dq_task_statistics_value::ds_dq_task_statistics_value_service_server::DsDqTaskStatisticsValueService;

#[tonic::async_trait]
impl DsDqTaskStatisticsValueService for AuroraRpcServer {
    async fn list_ds_dq_task_statistics_values(
        &self,
        _req: tonic::Request<proto::ds_dq_task_statistics_value::ListDsDqTaskStatisticsValuesRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_task_statistics_value::ListDsDqTaskStatisticsValuesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_task_statistics_value(
        &self,
        _req: tonic::Request<proto::ds_dq_task_statistics_value::GetDsDqTaskStatisticsValueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValue>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_dq_task_statistics_value(
        &self,
        _req: tonic::Request<proto::ds_dq_task_statistics_value::CreateDsDqTaskStatisticsValueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValue>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_dq_task_statistics_value(
        &self,
        _req: tonic::Request<proto::ds_dq_task_statistics_value::UpdateDsDqTaskStatisticsValueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValue>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_dq_task_statistics_value(
        &self,
        _req: tonic::Request<proto::ds_dq_task_statistics_value::DeleteDsDqTaskStatisticsValueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
