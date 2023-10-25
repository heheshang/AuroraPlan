use super::dao_service::AuroraRpcServer;
use proto::ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_service_server::DsDqRuleExecuteSqlService;

#[tonic::async_trait]
impl DsDqRuleExecuteSqlService for AuroraRpcServer {
    async fn list_ds_dq_rule_execute_sqls(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::ListDsDqRuleExecuteSqlsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::ListDsDqRuleExecuteSqlsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_rule_execute_sql(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::GetDsDqRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_rule_execute_sql(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::CreateDsDqRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_rule_execute_sql(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::UpdateDsDqRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_rule_execute_sql(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::DeleteDsDqRuleExecuteSqlRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
