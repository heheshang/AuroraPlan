use super::dao_service::AuroraRpcServer;
use lib_proto::ds_relation_process_instance::ds_relation_process_instance_service_server::DsRelationProcessInstanceService;

#[tonic::async_trait]
impl DsRelationProcessInstanceService for AuroraRpcServer {
    async fn list_ds_relation_process_instances(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_process_instance::ListDsRelationProcessInstancesRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_process_instance::ListDsRelationProcessInstancesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_process_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_process_instance::GetDsRelationProcessInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_process_instance::DsRelationProcessInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_process_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_process_instance::CreateDsRelationProcessInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_process_instance::DsRelationProcessInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_process_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_process_instance::UpdateDsRelationProcessInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_process_instance::DsRelationProcessInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_process_instance(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_process_instance::DeleteDsRelationProcessInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
