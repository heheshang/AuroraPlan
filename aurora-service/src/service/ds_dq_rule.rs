use super::dao_service::AuroraRpcServer;
use proto::ds_dq_rule::ds_dq_rule_service_server::DsDqRuleService;

#[tonic::async_trait]
impl DsDqRuleService for AuroraRpcServer {
    async fn list_ds_dq_rules(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::ListDsDqRulesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::ListDsDqRulesResponse>, tonic::Status> {
        todo!()
    }

    async fn get_ds_dq_rule(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::GetDsDqRuleRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRule>, tonic::Status> {
        todo!()
    }

    async fn create_ds_dq_rule(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::CreateDsDqRuleRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRule>, tonic::Status> {
        todo!()
    }

    async fn update_ds_dq_rule(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::UpdateDsDqRuleRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRule>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_dq_rule(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::DeleteDsDqRuleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
