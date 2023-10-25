use super::dao_service::AuroraRpcServer;
use proto::ds_dq_rule_input_entry::ds_dq_rule_input_entry_service_server::DsDqRuleInputEntryService;

#[tonic::async_trait]
impl DsDqRuleInputEntryService for AuroraRpcServer {
    async fn list_ds_dq_rule_input_entries(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::ListDsDqRuleInputEntriesRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::ListDsDqRuleInputEntriesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_rule_input_entry(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::GetDsDqRuleInputEntryRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntry>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_rule_input_entry(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::CreateDsDqRuleInputEntryRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntry>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_rule_input_entry(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::UpdateDsDqRuleInputEntryRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntry>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_rule_input_entry(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::DeleteDsDqRuleInputEntryRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
