use crate::{
    qrtz_blob_triggers::Model as QrtzBlobTriggerModel, qrtz_calendars::Model as QrtzCalendarModel,
    qrtz_cron_triggers::Model as QrtzCronTriggersModel, qrtz_fired_triggers::Model as QrtzFiredTriggersModel,
    qrtz_job_details::Model as QrtzJobDetailsModel, qrtz_locks::Model as QrtzLocksModel,
    qrtz_paused_trigger_grps::Model as QrtzPausedTriggerGrpsModel,
    qrtz_scheduler_state::Model as QrtzSchedulerStateModel, qrtz_simple_triggers::Model as QrtzSimpleTriggersModel,
    qrtz_simprop_triggers::Model as QrtzSimpropTriggerModel, qrtz_triggers::Model as QrtzTriggersModel,
    t_ds_access_token::Model as DsAccessTokenModel, t_ds_alert::Model as DsAlertModel,
    t_ds_alert_plugin_instance::Model as DsAlertPluginInstanceModel,
    t_ds_alert_send_status::Model as DsAlertSendStatusModel, t_ds_alertgroup::Model as DsAlertgroupModel,
    t_ds_audit_log::Model as DsAuditLogModel, t_ds_command::Model as DsCommandModel,
    t_ds_datasource::Model as DsDatasourceModel, t_ds_dq_comparison_type::Model as DsDqComparisonTypeModel,
    t_ds_dq_execute_result::Model as DsDqExecuteResultModel, t_ds_dq_rule::Model as DsDqRuleModel,
    t_ds_dq_rule_execute_sql::Model as DsDqRuleExecuteSqlModel,
    t_ds_dq_rule_input_entry::Model as DsDqRuleInputEntryModel,
    t_ds_dq_task_statistics_value::Model as DsDqTaskStatisticsValueModel,
    t_ds_environment::Model as DsEnvironmentModel,
    t_ds_environment_worker_group_relation::Model as DsEnvironmentWorkerGroupRelationModel,
    t_ds_error_command::Model as DsErrorCommandModel, t_ds_k8s::Model as DsK8sModel,
    t_ds_k8s_namespace::Model as DsK8sNamespaceModel, t_ds_plugin_define::Model as DsPluginDefineModel,
    t_ds_process_definition::Model as DsProcessDefinitionModel,
    t_ds_process_definition_log::Model as DsProcessDefinitionLogModel,
    t_ds_process_instance::Model as DsProcessInstanceModel,
    t_ds_process_task_relation::Model as DsProcessTaskRelationModel,
    t_ds_process_task_relation_log::Model as DsProcessTaskRelationLogModel, t_ds_project::Model as DsProjectModel,
    t_ds_project_parameter::Model as DsProjectParameterModel, t_ds_queue::Model as DsQueueModel,
    t_ds_relation_datasource_user::Model as DsRelationDatasourceUserModel,
    t_ds_relation_namespace_user::Model as DsRelationNamespaceUserModel,
    t_ds_relation_process_instance::Model as DsRelationProcessInstanceModel,
    t_ds_relation_project_user::Model as DsRelationProjectUserModel,
    t_ds_relation_resources_user::Model as DsRelationResourcesUserModel,
    t_ds_relation_rule_execute_sql::Model as DsRelationRuleExecuteSqlModel,
    t_ds_relation_rule_input_entry::Model as DsRelationRuleInputEntryModel,
    t_ds_relation_udfs_user::Model as DsRelationUdfsUserModel, t_ds_resources::Model as DsResourcesModel,
    t_ds_schedules::Model as DsSchedulesModel, t_ds_session::Model as DsSessionModel,
    t_ds_task_definition::Model as DsTaskDefinitionModel, t_ds_task_definition_log::Model as DsTaskDefinitionLogModel,
    t_ds_task_group::Model as DsTaskGroupModel, t_ds_task_group_queue::Model as DsTaskGroupQueueModel,
    t_ds_task_instance::Model as DsTaskInstanceModel, t_ds_tenant::Model as DsTenantModel,
    t_ds_udfs::Model as DsUdfsModel, t_ds_user::Model as DsUserModel, t_ds_version::Model as DsVersionModel,
    t_ds_worker_group::Model as DsWorkerGroupModel,
};

use proto::{
    ds_access_token::DsAccessToken, ds_alert::DsAlert, ds_alert_plugin_instance::DsAlertPluginInstance,
    ds_alert_send_status::DsAlertSendStatus, ds_alertgroup::DsAlertGroup, ds_audit_log::DsAuditLog,
    ds_command::DsCommand, ds_datasource::DsDatasource, ds_dq_comparison_type::DsDqComparisonType,
    ds_dq_execute_result::DsDqExecuteResult, ds_dq_rule::DsDqRule, ds_dq_rule_execute_sql::DsDqRuleExecuteSql,
    ds_dq_rule_input_entry::DsDqRuleInputEntry, ds_dq_task_statistics_value::DsDqTaskStatisticsValue,
    ds_environment::DsEnvironment, ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelation,
    ds_error_command::DsErrorCommand, ds_k8s::DsK8s, ds_k8s_namespace::DsK8sNamespace,
    ds_plugin_define::DsPluginDefine, ds_process_definition::DsProcessDefinition,
    ds_process_definition_log::DsProcessDefinitionLog, ds_process_instance::DsProcessInstance,
    ds_process_task_relation::DsProcessTaskRelation, ds_process_task_relation_log::DsProcessTaskRelationLog,
    ds_project::DsProject, ds_project_parameter::ProjectParameter, ds_queue::DsQueue,
    ds_relation_datasource_user::DsRelationDatasourceUser, ds_relation_namespace_user::DsRelationNamespaceUser,
    ds_relation_process_instance::DsRelationProcessInstance, ds_relation_project_user::DsRelationProjectUser,
    ds_relation_resources_user::DsRelationResourcesUser, ds_relation_rule_execute_sql::DsRelationRuleExecuteSql,
    ds_relation_rule_input::DsRelationRuleInput, ds_relation_udfs_user::DsRelationUdfsUser, ds_resources::DsResource,
    ds_schedules::DsSchedules, ds_session::DsSession, ds_task_definition::DsTaskDefinition,
    ds_task_definition_log::DsTaskDefinitionLog, ds_task_group::DsTaskGroup, ds_task_group_queue::DsTaskGroupQueue,
    ds_task_instance::DsTaskInstance, ds_tenant::DsTenant, ds_udfs::DsUdfs, ds_user::DsUser, ds_version::DsVersion,
    ds_worker_group::DsWorkerGroup, qrtz_blob_triggers::QrtzBlobTrigger, qrtz_calendars::QrtzCalendar,
    qrtz_cron_triggers::QrtzCronTriggers, qrtz_fired_triggers::QrtzFiredTriggers, qrtz_job_details::QrtzJobDetails,
    qrtz_locks::QrtzLocks, qrtz_paused_trigger_grps::QrtzPausedTriggerGrps, qrtz_scheduler_state::QrtzSchedulerState,
    qrtz_simple_triggers::QrtzSimpleTrigger, qrtz_simprop_triggers::QrtzSimpropTrigger, qrtz_triggers::QrtzTrigger,
};

macro_rules! impl_from {
    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(model: $from) -> Self {
                serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap()
            }
        }
    };
}
impl_from!(DsProjectParameterModel, ProjectParameter);
impl_from!(DsAccessTokenModel, DsAccessToken);
impl_from!(DsAlertModel, DsAlert);
impl_from!(DsAlertPluginInstanceModel, DsAlertPluginInstance);
impl_from!(DsAlertSendStatusModel, DsAlertSendStatus);
impl_from!(DsAlertgroupModel, DsAlertGroup);
impl_from!(DsAuditLogModel, DsAuditLog);
impl_from!(DsCommandModel, DsCommand);
impl_from!(DsDatasourceModel, DsDatasource);
impl_from!(DsDqComparisonTypeModel, DsDqComparisonType);
impl_from!(DsDqExecuteResultModel, DsDqExecuteResult);
impl_from!(DsDqRuleModel, DsDqRule);
impl_from!(DsDqRuleExecuteSqlModel, DsDqRuleExecuteSql);
impl_from!(DsDqRuleInputEntryModel, DsDqRuleInputEntry);
impl_from!(DsDqTaskStatisticsValueModel, DsDqTaskStatisticsValue);
impl_from!(DsEnvironmentModel, DsEnvironment);
impl_from!(DsEnvironmentWorkerGroupRelationModel, DsEnvironmentWorkerGroupRelation);
impl_from!(DsErrorCommandModel, DsErrorCommand);
impl_from!(DsK8sModel, DsK8s);
impl_from!(DsK8sNamespaceModel, DsK8sNamespace);
impl_from!(DsPluginDefineModel, DsPluginDefine);
impl_from!(DsProcessDefinitionModel, DsProcessDefinition);
impl_from!(DsProcessDefinitionLogModel, DsProcessDefinitionLog);
impl_from!(DsProcessInstanceModel, DsProcessInstance);
impl_from!(DsProcessTaskRelationModel, DsProcessTaskRelation);
impl_from!(DsProcessTaskRelationLogModel, DsProcessTaskRelationLog);
impl_from!(DsProjectModel, DsProject);
impl_from!(DsQueueModel, DsQueue);
impl_from!(DsRelationDatasourceUserModel, DsRelationDatasourceUser);
impl_from!(DsRelationNamespaceUserModel, DsRelationNamespaceUser);
impl_from!(DsRelationProcessInstanceModel, DsRelationProcessInstance);
impl_from!(DsRelationProjectUserModel, DsRelationProjectUser);
impl_from!(DsRelationResourcesUserModel, DsRelationResourcesUser);
impl_from!(DsRelationRuleExecuteSqlModel, DsRelationRuleExecuteSql);
impl_from!(DsRelationRuleInputEntryModel, DsRelationRuleInput);
impl_from!(DsRelationUdfsUserModel, DsRelationUdfsUser);
impl_from!(DsResourcesModel, DsResource);
impl_from!(DsSchedulesModel, DsSchedules);
impl_from!(DsSessionModel, DsSession);
impl_from!(DsTaskDefinitionModel, DsTaskDefinition);
impl_from!(DsTaskDefinitionLogModel, DsTaskDefinitionLog);
impl_from!(DsTaskGroupModel, DsTaskGroup);
impl_from!(DsTaskGroupQueueModel, DsTaskGroupQueue);
impl_from!(DsTaskInstanceModel, DsTaskInstance);
impl_from!(DsTenantModel, DsTenant);
impl_from!(DsUdfsModel, DsUdfs);
impl_from!(DsUserModel, DsUser);
impl_from!(DsVersionModel, DsVersion);
impl_from!(DsWorkerGroupModel, DsWorkerGroup);
impl_from!(QrtzBlobTriggerModel, QrtzBlobTrigger);
impl_from!(QrtzCalendarModel, QrtzCalendar);
impl_from!(QrtzCronTriggersModel, QrtzCronTriggers);
impl_from!(QrtzFiredTriggersModel, QrtzFiredTriggers);
impl_from!(QrtzJobDetailsModel, QrtzJobDetails);
impl_from!(QrtzLocksModel, QrtzLocks);
impl_from!(QrtzPausedTriggerGrpsModel, QrtzPausedTriggerGrps);
impl_from!(QrtzSchedulerStateModel, QrtzSchedulerState);
impl_from!(QrtzSimpleTriggersModel, QrtzSimpleTrigger);
impl_from!(QrtzTriggersModel, QrtzTrigger);
impl_from!(QrtzSimpropTriggerModel, QrtzSimpropTrigger);

// impl From<DsUserModel> for DsUser}; {
//     fn from(model: DsUserModel) -> Self {
//         serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap()

//         // Self {
//         //     id: model.id,
//         //     user_name: model.user_name,
//         //     user_password: model.user_password,
//         //     user_type: model.user_type,
//         //     email: model.email,
//         //     phone: model.phone,
//         //     tenant_id: model.tenant_id,
//         //     create_time: model.create_time.map(|x| x.to_string()),
//         //     update_time: model.update_time.map(|x| x.to_string()),
//         //     queue: model.queue,
//         //     state: model.state,
//         //     time_zone: model.time_zone,
//         // }
//     }
// }

// impl From<crate::qrtz_blob_triggers::Model> for QrtzBlobTrigger}; {
//     fn from(model: crate::qrtz_blob_triggers::Model) -> Self {
//         Self {
//             sched_name: model.sched_name,
//             trigger_name: model.trigger_name,
//             trigger_group: model.trigger_group,
//             blob_data: model.blob_data,
//         }
//     }
// }
