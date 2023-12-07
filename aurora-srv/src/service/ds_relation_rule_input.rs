use super::dao_service::AuroraRpcServer;
use proto::ds_relation_rule_input::ds_relation_rule_input_service_server::DsRelationRuleInputService;

#[tonic::async_trait]
impl DsRelationRuleInputService for AuroraRpcServer {
    async fn list_ds_relation_rule_inputs(
        &self,
        _req: tonic::Request<proto::ds_relation_rule_input::ListDsRelationRuleInputsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_input::ListDsRelationRuleInputsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_rule_input(
        &self,
        _req: tonic::Request<proto::ds_relation_rule_input::GetDsRelationRuleInputRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_relation_rule_input::DsRelationRuleInput>, tonic::Status> {
        todo!()
    }

    async fn create_ds_relation_rule_input(
        &self,
        _req: tonic::Request<proto::ds_relation_rule_input::CreateDsRelationRuleInputRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_relation_rule_input::DsRelationRuleInput>, tonic::Status> {
        todo!()
    }

    async fn update_ds_relation_rule_input(
        &self,
        _req: tonic::Request<proto::ds_relation_rule_input::UpdateDsRelationRuleInputRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_relation_rule_input::DsRelationRuleInput>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_relation_rule_input(
        &self,
        _req: tonic::Request<proto::ds_relation_rule_input::DeleteDsRelationRuleInputRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
