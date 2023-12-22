use super::dao_service::AuroraRpcServer;
use lib_proto::ds_dq_comparison_type::ds_dq_comparison_type_service_server::DsDqComparisonTypeService;

#[tonic::async_trait]
impl DsDqComparisonTypeService for AuroraRpcServer {
    async fn list_ds_dq_comparison_types(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_comparison_type::ListDsDqComparisonTypesRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_dq_comparison_type::ListDsDqComparisonTypesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_comparison_type(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_comparison_type::GetDsDqComparisonTypeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_comparison_type::DsDqComparisonType>, tonic::Status> {
        todo!()
    }

    async fn create_ds_dq_comparison_type(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_comparison_type::CreateDsDqComparisonTypeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_comparison_type::DsDqComparisonType>, tonic::Status> {
        todo!()
    }

    async fn update_ds_dq_comparison_type(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_comparison_type::UpdateDsDqComparisonTypeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_dq_comparison_type::DsDqComparisonType>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_dq_comparison_type(
        &self,
        _req: tonic::Request<lib_proto::ds_dq_comparison_type::DeleteDsDqComparisonTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
