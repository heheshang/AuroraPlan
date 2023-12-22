use lib_conifg::api_config::Settings;
use lib_proto::{
    ds_access_token::ds_access_token_service_client::DsAccessTokenServiceClient,
    ds_alert::ds_alert_service_client::DsAlertServiceClient,
    ds_alert_plugin_instance::ds_alert_plugin_instance_service_client::DsAlertPluginInstanceServiceClient,
    ds_alert_send_status::ds_alert_send_status_service_client::DsAlertSendStatusServiceClient,
    ds_alertgroup::ds_alert_group_service_client::DsAlertGroupServiceClient,
    ds_audit_log::ds_audit_log_service_client::DsAuditLogServiceClient,
    ds_cluster::ds_cluster_service_client::DsClusterServiceClient,
    ds_command::ds_command_service_client::DsCommandServiceClient,
    ds_datasource::ds_datasource_service_client::DsDatasourceServiceClient,
    ds_dq_comparison_type::ds_dq_comparison_type_service_client::DsDqComparisonTypeServiceClient,
    ds_dq_execute_result::ds_dq_execute_result_service_client::DsDqExecuteResultServiceClient,
    ds_dq_rule::ds_dq_rule_service_client::DsDqRuleServiceClient,
    ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_service_client::DsDqRuleExecuteSqlServiceClient,
    ds_dq_rule_input_entry::ds_dq_rule_input_entry_service_client::DsDqRuleInputEntryServiceClient,
    ds_dq_task_statistics_value::ds_dq_task_statistics_value_service_client::DsDqTaskStatisticsValueServiceClient,
    ds_environment::ds_environment_service_client::DsEnvironmentServiceClient,
    ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_client::DsEnvironmentWorkerGroupRelationServiceClient,
    ds_error_command::ds_error_command_service_client::DsErrorCommandServiceClient,
    ds_k8s::ds_k8s_service_client::DsK8sServiceClient,
    ds_k8s_namespace::ds_k8s_namespace_service_client::DsK8sNamespaceServiceClient,
    ds_plugin_define::ds_plugin_define_service_client::DsPluginDefineServiceClient,
    ds_process_definition::ds_process_definition_service_client::DsProcessDefinitionServiceClient,
    ds_process_definition_log::ds_process_definition_log_service_client::DsProcessDefinitionLogServiceClient,
    ds_process_instance::ds_process_instance_service_client::DsProcessInstanceServiceClient,
    ds_process_task_relation::ds_process_task_relation_service_client::DsProcessTaskRelationServiceClient,
    ds_process_task_relation_log::ds_process_task_relation_log_service_client::DsProcessTaskRelationLogServiceClient,
    ds_project::ds_project_service_client::DsProjectServiceClient,
    ds_project_parameter::project_parameter_service_client::ProjectParameterServiceClient,
    ds_queue::ds_queue_service_client::DsQueueServiceClient,
    ds_relation_datasource_user::ds_relation_datasource_user_service_client::DsRelationDatasourceUserServiceClient,
    ds_relation_namespace_user::ds_relation_namespace_user_service_client::DsRelationNamespaceUserServiceClient,
    ds_relation_process_instance::ds_relation_process_instance_service_client::DsRelationProcessInstanceServiceClient,
    ds_relation_project_user::ds_relation_project_user_service_client::DsRelationProjectUserServiceClient,
    ds_relation_resources_user::ds_relation_resources_user_service_client::DsRelationResourcesUserServiceClient,
    ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_service_client::DsRelationRuleExecuteSqlServiceClient,
    ds_relation_rule_input::ds_relation_rule_input_service_client::DsRelationRuleInputServiceClient,
    ds_relation_udfs_user::ds_relation_udfs_user_service_client::DsRelationUdfsUserServiceClient,
    ds_resources::ds_resource_service_client::DsResourceServiceClient,
    ds_schedules::ds_schedules_service_client::DsSchedulesServiceClient,
    ds_session::ds_session_service_client::DsSessionServiceClient,
    ds_task_definition::ds_task_definition_service_client::DsTaskDefinitionServiceClient,
    ds_task_definition_log::ds_task_definition_log_service_client::DsTaskDefinitionLogServiceClient,
    ds_task_group::ds_task_group_service_client::DsTaskGroupServiceClient,
    ds_task_group_queue::ds_task_group_queue_service_client::DsTaskGroupQueueServiceClient,
    ds_task_instance::ds_task_instance_service_client::DsTaskInstanceServiceClient,
    ds_tenant::ds_tenant_service_client::DsTenantServiceClient, ds_udfs::ds_udfs_service_client::DsUdfsServiceClient,
    ds_user::ds_user_service_client::DsUserServiceClient,
    ds_version::ds_version_service_client::DsVersionServiceClient,
    ds_worker_group::ds_worker_group_service_client::DsWorkerGroupServiceClient,
    qrtz_blob_triggers::qrtz_blob_trigger_service_client::QrtzBlobTriggerServiceClient,
    qrtz_calendars::qrtz_calendar_service_client::QrtzCalendarServiceClient,
    qrtz_cron_triggers::qrtz_cron_triggers_service_client::QrtzCronTriggersServiceClient,
    qrtz_fired_triggers::qrtz_fired_triggers_service_client::QrtzFiredTriggersServiceClient,
    qrtz_job_details::qrtz_job_details_service_client::QrtzJobDetailsServiceClient,
    qrtz_locks::qrtz_locks_service_client::QrtzLocksServiceClient,
    qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_service_client::QrtzPausedTriggerGrpsServiceClient,
    qrtz_scheduler_state::qrtz_scheduler_state_service_client::QrtzSchedulerStateServiceClient,
    qrtz_simple_triggers::qrtz_simple_trigger_service_client::QrtzSimpleTriggerServiceClient,
    qrtz_simprop_triggers::qrtz_simprop_trigger_service_client::QrtzSimpropTriggerServiceClient,
    qrtz_triggers::qrtz_trigger_service_client::QrtzTriggerServiceClient,
};
use std::cell::RefCell;
use tonic::Request;
use tonic_health::pb::{health_client::HealthClient, HealthCheckRequest};
use tracing::{debug, error, warn};

use lib_common::core_error::error::AuroraData;
use lib_common::{core_error::error::Error, core_results::results::Result};
thread_local! {
    pub static SETTINGS:RefCell<Settings> = RefCell::new(Settings::new().unwrap());
}

macro_rules! build_client {
    ($contanst_name:ident, $fn_name:ident, $service_type:ident) => {
        pub static $contanst_name: tokio::sync::OnceCell<Result<$service_type<tonic::transport::Channel>>> =
            tokio::sync::OnceCell::const_new();

        pub async fn $fn_name() -> Result<$service_type<tonic::transport::Channel>> {
            let host = SETTINGS.with(|settings| settings.borrow().service.host.clone());
            let port = SETTINGS.with(|settings| settings.borrow().service.port.clone());

            loop {
                let res = check_client().await;
                match res {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }

            let addr_str = format!("http://{host}:{port}").clone();
            debug!("addr_str:{}", addr_str);
            let addr = tonic::transport::Endpoint::from_shared(addr_str);
            let client = match $service_type::connect(addr.unwrap()).await {
                Ok(client) => Ok(client),
                Err(_) => {
                    return Err(Error::InternalServerErrorArgs(AuroraData::Null, None));
                }
            };
            match $contanst_name.get_or_init(|| async { client }).await {
                Ok(client) => Ok(client.clone()),
                Err(_) => {
                    return Err(Error::InternalServerErrorArgs(AuroraData::Null, None));
                }
            }
        }
    };
}
#[allow(unused)]
async fn check_client() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let host = SETTINGS.with(|settings| settings.borrow().service.host.clone());
    let port = SETTINGS.with(|settings| settings.borrow().service.port);
    loop {
        debug!("host:{},port:{}", host, port);
        let conn = tonic::transport::Endpoint::new(format!("http://{}:{}", host, port))?
            .connect()
            .await?;

        let mut client = HealthClient::new(conn);
        let request = Request::new(HealthCheckRequest {
            service: "helloworld.Greeter".into(),
        });
        match client.check(request).await {
            Ok(response) => {
                let status = response.into_inner().status();
                match status {
                    tonic_health::pb::health_check_response::ServingStatus::Unknown => {
                        warn!("Unknown!")
                    }
                    tonic_health::pb::health_check_response::ServingStatus::Serving => {
                        break;
                    }
                    tonic_health::pb::health_check_response::ServingStatus::NotServing => {
                        warn!("Not Serving!")
                    }
                    tonic_health::pb::health_check_response::ServingStatus::ServiceUnknown => {
                        warn!("ServiceUnknown!")
                    }
                }
            }
            Err(status) => {
                match status.code() {
                    tonic::Code::Unimplemented => error!(
                        "error: this server does not implement the grpc health lib_protocol (grpc.health.v1.Health): \
                         {}",
                        status.message()
                    ),
                    tonic::Code::DeadlineExceeded => error!("timeout: health rpc did not complete within 1 second"),
                    _ => error!("error: health rpc failed: {}", status.message()),
                };
                continue;
            }
        }
    }
    Ok(())
}
build_client!(
    _DS_ACCESS_TOKEN,
    _ds_access_token_service_client,
    DsAccessTokenServiceClient
);
build_client!(_DS_ALERT, _ds_alert_service_client, DsAlertServiceClient);
build_client!(
    _DS_ALERT_PLUGIN_INSTANCE,
    _ds_alert_plugin_instance_service_client,
    DsAlertPluginInstanceServiceClient
);
build_client!(
    _DS_ALERT_SEND_STATUS,
    _ds_alert_send_status_service_client,
    DsAlertSendStatusServiceClient
);
build_client!(
    _DS_ALERTGROUP,
    _ds_alert_group_service_client,
    DsAlertGroupServiceClient
);
build_client!(_DS_CLUSTER, _ds_cluster_service_client, DsClusterServiceClient);
build_client!(_DS_AUDIT_LOG, _ds_audit_log_service_client, DsAuditLogServiceClient);
build_client!(_DS_COMMAND, _ds_command_service_client, DsCommandServiceClient);
build_client!(_DS_DATASOURCE, _ds_datasource_service_client, DsDatasourceServiceClient);
build_client!(
    _DS_DQ_COMPARISON_TYPE,
    _ds_dq_comparison_type_service_client,
    DsDqComparisonTypeServiceClient
);
build_client!(
    _DS_DQ_EXECUTE_RESULT,
    _ds_dq_execute_result_service_client,
    DsDqExecuteResultServiceClient
);
build_client!(_DS_DQ_RULE, _ds_dq_rule_service_client, DsDqRuleServiceClient);
build_client!(
    _DS_DQ_RULE_EXECUTE_SQL,
    _ds_dq_rule_execute_sql_service_client,
    DsDqRuleExecuteSqlServiceClient
);
build_client!(
    _DS_DQ_RULE_INPUT_ENTRY,
    _ds_dq_rule_input_entry_service_client,
    DsDqRuleInputEntryServiceClient
);
build_client!(
    _DS_DQ_TASK_STATISTICS_VALUE,
    _ds_dq_task_statistics_value_service_client,
    DsDqTaskStatisticsValueServiceClient
);
build_client!(
    _DS_ENVIRONMENT,
    _ds_environment_service_client,
    DsEnvironmentServiceClient
);
build_client!(
    _DS_ENVIRONMENT_WORKER_GROUP_RELATION,
    _ds_environment_worker_group_relation_service_client,
    DsEnvironmentWorkerGroupRelationServiceClient
);
build_client!(
    _DS_ERROR_COMMAND,
    _ds_error_command_service_client,
    DsErrorCommandServiceClient
);
build_client!(_DS_K8S, _ds_k8s_service_client, DsK8sServiceClient);
build_client!(
    _DS_K8S_NAMESPACE,
    _ds_k8s_namespace_service_client,
    DsK8sNamespaceServiceClient
);
build_client!(
    _DS_PLUGIN_DEFINE,
    _ds_plugin_define_service_client,
    DsPluginDefineServiceClient
);
build_client!(
    _DS_PROCESS_DEFINITION,
    _ds_process_definition_service_client,
    DsProcessDefinitionServiceClient
);
build_client!(
    _DS_PROCESS_DEFINITION_LOG,
    _ds_process_definition_log_service_client,
    DsProcessDefinitionLogServiceClient
);
build_client!(
    _DS_PROCESS_INSTANCE,
    _ds_process_instance_service_client,
    DsProcessInstanceServiceClient
);
build_client!(
    _DS_PROCESS_TASK_RELATION,
    _ds_process_task_relation_service_client,
    DsProcessTaskRelationServiceClient
);
build_client!(
    _DS_PROCESS_TASK_RELATION_LOG,
    _ds_process_task_relation_log_service_client,
    DsProcessTaskRelationLogServiceClient
);
build_client!(_DS_PROJECT, _ds_project_service_client, DsProjectServiceClient);
build_client!(
    _DS_PROJECT_PARAMETER,
    _ds_project_parameter_service_client,
    ProjectParameterServiceClient
);
build_client!(_DS_QUEUE, _ds_queue_service_client, DsQueueServiceClient);
build_client!(
    _DS_RELATION_DATASOURCE_USER,
    _ds_relation_datasource_user_service_client,
    DsRelationDatasourceUserServiceClient
);
build_client!(
    _DS_RELATION_NAMESPACE_USER,
    _ds_relation_namespace_user_service_client,
    DsRelationNamespaceUserServiceClient
);
build_client!(
    _DS_RELATION_PROCESS_INSTANCE,
    _ds_relation_process_instance_service_client,
    DsRelationProcessInstanceServiceClient
);
build_client!(
    _DS_RELATION_PROJECT_USER,
    _ds_relation_project_user_service_client,
    DsRelationProjectUserServiceClient
);
build_client!(
    _DS_RELATION_RESOURCES_USER,
    _ds_relation_resources_user_service_client,
    DsRelationResourcesUserServiceClient
);
build_client!(
    _DS_RELATION_RULE_EXECUTE_SQL,
    _ds_relation_rule_execute_sql_service_client,
    DsRelationRuleExecuteSqlServiceClient
);
build_client!(
    _DS_RELATION_RULE_INPUT,
    _ds_relation_rule_input_service_client,
    DsRelationRuleInputServiceClient
);
build_client!(
    _DS_RELATION_UDFS_USER,
    _ds_relation_udfs_user_service_client,
    DsRelationUdfsUserServiceClient
);
build_client!(_DS_RESOURCES, _ds_resource_service_client, DsResourceServiceClient);
build_client!(_DS_SCHEDULES, _ds_schedules_service_client, DsSchedulesServiceClient);
build_client!(_DS_SESSION, _ds_session_service_client, DsSessionServiceClient);
build_client!(
    _DS_TASK_DEFINITION,
    _ds_task_definition_service_client,
    DsTaskDefinitionServiceClient
);
build_client!(
    _DS_TASK_DEFINITION_LOG,
    _ds_task_definition_log_service_client,
    DsTaskDefinitionLogServiceClient
);
build_client!(_DS_TASK_GROUP, _ds_task_group_service_client, DsTaskGroupServiceClient);
build_client!(
    _DS_TASK_GROUP_QUEUE,
    _ds_task_group_queue_service_client,
    DsTaskGroupQueueServiceClient
);
build_client!(
    _DS_TASK_INSTANCE,
    _ds_task_instance_service_client,
    DsTaskInstanceServiceClient
);
build_client!(_DS_TENANT, _ds_tenant_service_client, DsTenantServiceClient);
build_client!(_DS_UDFS, _ds_udfs_service_client, DsUdfsServiceClient);
build_client!(_DS_USER, _ds_user_service_client, DsUserServiceClient);
build_client!(_DS_VERSION, _ds_version_service_client, DsVersionServiceClient);
build_client!(
    _DS_WORKER_GROUP,
    _ds_worker_group_service_client,
    DsWorkerGroupServiceClient
);
build_client!(
    _QRTZ_BLOB_TRIGGER_CLIENT,
    _qrtz_blob_trigger_service_client,
    QrtzBlobTriggerServiceClient
);
build_client!(
    _QRTZ_SCHEDULER_STATE_CLIENT,
    _qrtz_scheduler_state_service_client,
    QrtzSchedulerStateServiceClient
);
build_client!(
    _QRTZ_SIMPLE_TRIGGER_SERVICE_CLIENT,
    _qrtz_simple_trigger_client,
    QrtzSimpleTriggerServiceClient
);
build_client!(
    _QRTZ_SIMPROP_TRIGGER_SERVICE_CLIENT,
    _qrtz_simprop_trigger_client,
    QrtzSimpropTriggerServiceClient
);
build_client!(
    _QRTZ_TRIGGER_SERVICE_CLIENT,
    _qrtz_trigger_client,
    QrtzTriggerServiceClient
);
build_client!(
    _QRTZ_CALENDARS,
    _qrtz_calendar_service_client,
    QrtzCalendarServiceClient
);
build_client!(
    _QRTZ_CRON_TRIGGERS,
    _qrtz_cron_triggers_service_client,
    QrtzCronTriggersServiceClient
);
build_client!(
    _QRTZ_FIRED_TRIGGERS,
    _qrtz_fired_triggers_service_client,
    QrtzFiredTriggersServiceClient
);
build_client!(
    _QRTZ_JOB_DETAILS,
    _qrtz_job_details_service_client,
    QrtzJobDetailsServiceClient
);
build_client!(_QRTZ_LOCKS, _qrtz_locks_service_client, QrtzLocksServiceClient);
build_client!(
    _QRTZ_PAUSED_TRIGGER_GRPS,
    _qrtz_paused_trigger_grps_service_client,
    QrtzPausedTriggerGrpsServiceClient
);

// pub static USER_SERVICE: OnceCell<Result<DsUserServiceClient<Channel>>> =
//     OnceCell::const_new();

// pub async fn get_client() -> Result<DsUserServiceClient<Channel>> {
//     dotenvy::dotenv().ok();
//     let host = env::var("aurora_DAO_HOST")
//         .expect("HOST is not set in .env file")
//         .clone();
//     let port = env::var("aurora_DAO_PORT")
//         .expect("PORT is not set in .env file")
//         .clone();
//     let addr_str = format!("http://{host}:{port}").clone();
//     let addr = Endpoint::from_shared(addr_str);
//     match DsUserServiceClient::connect(addr.unwrap()).await {
//         Ok(client) => Ok(client),
//         Err(e) => Err(anyhow::Error::new(e)),
//     }
// }
