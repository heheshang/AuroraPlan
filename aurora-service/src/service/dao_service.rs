use proto::{
    ds_access_token::ds_access_token_service_server::DsAccessTokenServiceServer,
    ds_alert::ds_alert_service_server::DsAlertServiceServer,
    ds_alert_plugin_instance::ds_alert_plugin_instance_service_server::DsAlertPluginInstanceServiceServer,
    ds_alert_send_status::ds_alert_send_status_service_server::DsAlertSendStatusServiceServer,
    ds_alertgroup::ds_alert_group_service_server::DsAlertGroupServiceServer,
    ds_audit_log::ds_audit_log_service_server::DsAuditLogServiceServer,
    ds_command::ds_command_service_server::DsCommandServiceServer,
    ds_datasource::ds_datasource_service_server::DsDatasourceServiceServer,
    ds_dq_comparison_type::ds_dq_comparison_type_service_server::DsDqComparisonTypeServiceServer,
    ds_dq_execute_result::ds_dq_execute_result_service_server::DsDqExecuteResultServiceServer,
    ds_dq_rule::ds_dq_rule_service_server::DsDqRuleServiceServer,
    ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_service_server::DsDqRuleExecuteSqlServiceServer,
    ds_dq_rule_input_entry::ds_dq_rule_input_entry_service_server::DsDqRuleInputEntryServiceServer,
    ds_dq_task_statistics_value::ds_dq_task_statistics_value_service_server::DsDqTaskStatisticsValueServiceServer,
    ds_environment::ds_environment_service_server::DsEnvironmentServiceServer,
    ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_server::DsEnvironmentWorkerGroupRelationServiceServer,
    ds_error_command::ds_error_command_service_server::DsErrorCommandServiceServer,
    ds_k8s::ds_k8s_service_server::DsK8sServiceServer,
    ds_k8s_namespace::ds_k8s_namespace_service_server::DsK8sNamespaceServiceServer,
    ds_plugin_define::ds_plugin_define_service_server::DsPluginDefineServiceServer,
    ds_process_definition::ds_process_definition_service_server::DsProcessDefinitionServiceServer,
    ds_process_definition_log::ds_process_definition_log_service_server::DsProcessDefinitionLogServiceServer,
    ds_process_instance::ds_process_instance_service_server::DsProcessInstanceServiceServer,
    ds_process_task_relation::ds_process_task_relation_service_server::DsProcessTaskRelationServiceServer,
    ds_process_task_relation_log::ds_process_task_relation_log_service_server::DsProcessTaskRelationLogServiceServer,
    ds_project::ds_project_service_server::DsProjectServiceServer,
    ds_project_parameter::project_parameter_service_server::ProjectParameterServiceServer,
    ds_queue::ds_queue_service_server::DsQueueServiceServer,
    ds_relation_datasource_user::ds_relation_datasource_user_service_server::DsRelationDatasourceUserServiceServer,
    ds_relation_namespace_user::ds_relation_namespace_user_service_server::DsRelationNamespaceUserServiceServer,
    ds_relation_process_instance::ds_relation_process_instance_service_server::DsRelationProcessInstanceServiceServer,
    ds_relation_project_user::ds_relation_project_user_service_server::DsRelationProjectUserServiceServer,
    ds_relation_resources_user::ds_relation_resources_user_service_server::DsRelationResourcesUserServiceServer,
    ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_service_server::DsRelationRuleExecuteSqlServiceServer,
    ds_relation_rule_input::ds_relation_rule_input_service_server::DsRelationRuleInputServiceServer,
    ds_relation_udfs_user::ds_relation_udfs_user_service_server::DsRelationUdfsUserServiceServer,
    ds_resources::ds_resource_service_server::DsResourceServiceServer,
    ds_schedules::ds_schedules_service_server::DsSchedulesServiceServer,
    ds_session::ds_session_service_server::DsSessionServiceServer,
    ds_task_definition::ds_task_definition_service_server::DsTaskDefinitionServiceServer,
    ds_task_definition_log::ds_task_definition_log_service_server::DsTaskDefinitionLogServiceServer,
    ds_task_group::ds_task_group_service_server::DsTaskGroupServiceServer,
    ds_task_group_queue::ds_task_group_queue_service_server::DsTaskGroupQueueServiceServer,
    ds_task_instance::ds_task_instance_service_server::DsTaskInstanceServiceServer,
    ds_tenant::ds_tenant_service_server::DsTenantServiceServer, ds_udfs::ds_udfs_service_server::DsUdfsServiceServer,
    ds_user::ds_user_service_server::DsUserServiceServer,
    ds_version::ds_version_service_server::DsVersionServiceServer,
    ds_worker_group::ds_worker_group_service_server::DsWorkerGroupServiceServer,
    qrtz_blob_triggers::qrtz_blob_trigger_service_server::QrtzBlobTriggerServiceServer,
    qrtz_calendars::qrtz_calendar_service_server::QrtzCalendarServiceServer,
    qrtz_cron_triggers::qrtz_cron_triggers_service_server::QrtzCronTriggersServiceServer,
    qrtz_fired_triggers::qrtz_fired_triggers_service_server::QrtzFiredTriggersServiceServer,
    qrtz_job_details::qrtz_job_details_service_server::QrtzJobDetailsServiceServer,
    qrtz_locks::qrtz_locks_service_server::QrtzLocksServiceServer,
    qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_service_server::QrtzPausedTriggerGrpsServiceServer,
    qrtz_scheduler_state::qrtz_scheduler_state_service_server::QrtzSchedulerStateServiceServer,
    qrtz_simple_triggers::qrtz_simple_trigger_service_server::QrtzSimpleTriggerServiceServer,
    qrtz_simprop_triggers::qrtz_simprop_trigger_service_server::QrtzSimpropTriggerServiceServer,
    qrtz_triggers::qrtz_trigger_service_server::QrtzTriggerServiceServer,
};
use sea_orm::DatabaseConnection;

#[derive(Default, Debug, Clone)]
pub struct AuroraRpcServer {
    pub db: DatabaseConnection,
}
macro_rules! create_service {
    ($fn_name:ident, $service_type:ident) => {
        pub fn $fn_name(self) -> $service_type<Self> {
            $service_type::new(self)
        }
    };
}

impl AuroraRpcServer {
    create_service!(ds_access_token, DsAccessTokenServiceServer);

    create_service!(ds_alert, DsAlertServiceServer);

    create_service!(ds_alert_plugin_instance, DsAlertPluginInstanceServiceServer);

    create_service!(ds_alert_send_status, DsAlertSendStatusServiceServer);

    create_service!(ds_alertgroup, DsAlertGroupServiceServer);

    create_service!(ds_audit_log, DsAuditLogServiceServer);

    create_service!(ds_command, DsCommandServiceServer);

    create_service!(ds_datasource, DsDatasourceServiceServer);

    create_service!(ds_dq_comparison_type, DsDqComparisonTypeServiceServer);

    create_service!(ds_dq_execute_result, DsDqExecuteResultServiceServer);

    create_service!(ds_dq_rule, DsDqRuleServiceServer);

    create_service!(ds_dq_rule_execute_sql, DsDqRuleExecuteSqlServiceServer);

    create_service!(ds_dq_rule_input_entry, DsDqRuleInputEntryServiceServer);

    create_service!(ds_dq_task_statistics_value, DsDqTaskStatisticsValueServiceServer);

    create_service!(ds_environment, DsEnvironmentServiceServer);

    create_service!(
        ds_environment_worker_group_relation,
        DsEnvironmentWorkerGroupRelationServiceServer
    );

    create_service!(ds_error_command, DsErrorCommandServiceServer);

    create_service!(ds_k8s, DsK8sServiceServer);

    create_service!(ds_k8s_namespace, DsK8sNamespaceServiceServer);

    create_service!(ds_plugin_define, DsPluginDefineServiceServer);

    create_service!(ds_process_definition, DsProcessDefinitionServiceServer);

    create_service!(ds_process_definition_log, DsProcessDefinitionLogServiceServer);

    create_service!(ds_process_instance, DsProcessInstanceServiceServer);

    create_service!(ds_process_task_relation, DsProcessTaskRelationServiceServer);

    create_service!(ds_process_task_relation_log, DsProcessTaskRelationLogServiceServer);

    create_service!(ds_project, DsProjectServiceServer);
    create_service!(ds_project_parameter, ProjectParameterServiceServer);

    create_service!(ds_queue, DsQueueServiceServer);

    create_service!(ds_relation_datasource_user, DsRelationDatasourceUserServiceServer);

    create_service!(ds_relation_namespace_user, DsRelationNamespaceUserServiceServer);

    create_service!(ds_relation_process_instance, DsRelationProcessInstanceServiceServer);

    create_service!(ds_relation_project_user, DsRelationProjectUserServiceServer);

    create_service!(ds_relation_resources_user, DsRelationResourcesUserServiceServer);

    create_service!(ds_relation_rule_execute_sql, DsRelationRuleExecuteSqlServiceServer);

    create_service!(ds_relation_rule_input, DsRelationRuleInputServiceServer);

    create_service!(ds_relation_udfs_user, DsRelationUdfsUserServiceServer);

    create_service!(ds_resources, DsResourceServiceServer);

    create_service!(ds_schedules, DsSchedulesServiceServer);

    create_service!(ds_session, DsSessionServiceServer);

    create_service!(ds_task_definition, DsTaskDefinitionServiceServer);

    create_service!(ds_task_definition_log, DsTaskDefinitionLogServiceServer);

    create_service!(ds_task_group, DsTaskGroupServiceServer);

    create_service!(ds_task_group_queue, DsTaskGroupQueueServiceServer);

    create_service!(ds_task_instance, DsTaskInstanceServiceServer);

    create_service!(ds_tenant, DsTenantServiceServer);

    create_service!(ds_udfs, DsUdfsServiceServer);

    create_service!(ds_user, DsUserServiceServer);

    create_service!(ds_version, DsVersionServiceServer);

    create_service!(ds_worker_group, DsWorkerGroupServiceServer);

    create_service!(qrtz_blob_triggers, QrtzBlobTriggerServiceServer);

    create_service!(qrtz_calendars, QrtzCalendarServiceServer);

    create_service!(qrtz_cron_triggers, QrtzCronTriggersServiceServer);

    create_service!(qrtz_fired_triggers, QrtzFiredTriggersServiceServer);

    create_service!(qrtz_job_details, QrtzJobDetailsServiceServer);

    create_service!(qrtz_locks, QrtzLocksServiceServer);

    create_service!(qrtz_paused_trigger_grps, QrtzPausedTriggerGrpsServiceServer);

    create_service!(qrtz_scheduler_state, QrtzSchedulerStateServiceServer);

    create_service!(qrtz_simple_triggers, QrtzSimpleTriggerServiceServer);

    create_service!(qrtz_simprop_triggers, QrtzSimpropTriggerServiceServer);

    create_service!(qrtz_triggers, QrtzTriggerServiceServer);

    pub fn new(conn: DatabaseConnection) -> Self {
        Self { db: conn }
    }
}
