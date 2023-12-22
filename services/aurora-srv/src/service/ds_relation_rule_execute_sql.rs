use super::dao_service::AuroraRpcServer;
use lib_proto::ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_service_server::DsRelationRuleExecuteSqlService;

#[tonic::async_trait]
impl DsRelationRuleExecuteSqlService for AuroraRpcServer {
    async fn list_ds_relation_rule_execute_sqls(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_rule_execute_sql::ListDsRelationRuleExecuteSqlsRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_rule_execute_sql::ListDsRelationRuleExecuteSqlsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_rule_execute_sql(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_rule_execute_sql::GetDsRelationRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_rule_execute_sql(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_rule_execute_sql::CreateDsRelationRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_rule_execute_sql(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_rule_execute_sql::UpdateDsRelationRuleExecuteSqlRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSql>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_rule_execute_sql(
        &self,
        _req: tonic::Request<lib_proto::ds_relation_rule_execute_sql::DeleteDsRelationRuleExecuteSqlRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
