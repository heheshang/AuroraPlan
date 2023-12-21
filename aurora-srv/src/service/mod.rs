use std::time::Duration;

use proto::helloworld::greeter_server::GreeterServer;
use sqlx::PgPool;
use tonic::transport::{server::Router, Server};
use tonic_health::server::HealthReporter;
use tracing::info;

use self::{dao_service::AuroraRpcServer, my_greeter::MyGreeter};

pub mod dao_service;
pub mod ds_access_token;
pub mod ds_alert;
pub mod ds_alert_plugin_instance;
pub mod ds_alert_send_status;
pub mod ds_alertgroup;
pub mod ds_audit_log;
pub mod ds_cluster;
pub mod ds_command;
pub mod ds_datasource;
pub mod ds_dq_comparison_type;
pub mod ds_dq_execute_result;
pub mod ds_dq_rule;
pub mod ds_dq_rule_execute_sql;
pub mod ds_dq_rule_input_entry;
pub mod ds_dq_task_statistics_value;
pub mod ds_environment;
pub mod ds_environment_worker_group_relation;
pub mod ds_error_command;
pub mod ds_k8s;
pub mod ds_k8s_namespace;
pub mod ds_plugin_define;
pub mod ds_process_definition;
pub mod ds_process_definition_log;
pub mod ds_process_instance;
pub mod ds_process_task_relation;
pub mod ds_process_task_relation_log;
pub mod ds_project;
pub mod ds_project_parameter;
pub mod ds_queue;
pub mod ds_relation_datasource_user;
pub mod ds_relation_namespace_user;
pub mod ds_relation_process_instance;
pub mod ds_relation_project_user;
pub mod ds_relation_resources_user;
pub mod ds_relation_rule_execute_sql;
pub mod ds_relation_rule_input;
pub mod ds_relation_udfs_user;
pub mod ds_resources;
pub mod ds_schedules;
pub mod ds_session;
pub mod ds_task_definition;
pub mod ds_task_definition_log;
pub mod ds_task_group;
pub mod ds_task_group_queue;
pub mod ds_task_instance;
pub mod ds_tenant;
pub mod ds_udfs;
pub mod ds_user;
pub mod ds_version;
pub mod ds_worker_group;
pub mod my_greeter;
pub mod qrtz_blob_triggers;
pub mod qrtz_calendars;
pub mod qrtz_cron_triggers;
pub mod qrtz_fired_triggers;
pub mod qrtz_job_details;
pub mod qrtz_locks;
pub mod qrtz_paused_trigger_grps;
pub mod qrtz_scheduler_state;
pub mod qrtz_simple_triggers;
pub mod qrtz_simprop_triggers;
pub mod qrtz_triggers;

pub(crate) async fn build_router(pool: PgPool) -> Router {
    info!("build_router");
    let grpc_server = AuroraRpcServer::new(pool);
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<GreeterServer<MyGreeter>>().await;
    // tokio::spawn(twiddle_service_status(health_reporter));
    let greeter = GreeterServer::new(MyGreeter::default());
    Server::builder()
        .add_service(health_service)
        .add_service(greeter)
        .add_optional_service(Some(grpc_server.clone().ds_access_token()))
        .add_optional_service(Some(grpc_server.clone().ds_alert()))
        .add_optional_service(Some(grpc_server.clone().ds_alert_plugin_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_alert_send_status()))
        .add_optional_service(Some(grpc_server.clone().ds_alertgroup()))
        .add_optional_service(Some(grpc_server.clone().ds_cluster()))
        .add_optional_service(Some(grpc_server.clone().ds_audit_log()))
        .add_optional_service(Some(grpc_server.clone().ds_command()))
        .add_optional_service(Some(grpc_server.clone().ds_datasource()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_comparison_type()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_execute_result()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule_execute_sql()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_rule_input_entry()))
        .add_optional_service(Some(grpc_server.clone().ds_dq_task_statistics_value()))
        .add_optional_service(Some(grpc_server.clone().ds_environment()))
        .add_optional_service(Some(grpc_server.clone().ds_environment_worker_group_relation()))
        .add_optional_service(Some(grpc_server.clone().ds_error_command()))
        .add_optional_service(Some(grpc_server.clone().ds_k8s()))
        .add_optional_service(Some(grpc_server.clone().ds_k8s_namespace()))
        .add_optional_service(Some(grpc_server.clone().ds_plugin_define()))
        .add_optional_service(Some(grpc_server.clone().ds_process_definition()))
        .add_optional_service(Some(grpc_server.clone().ds_process_definition_log()))
        .add_optional_service(Some(grpc_server.clone().ds_process_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_process_task_relation()))
        .add_optional_service(Some(grpc_server.clone().ds_process_task_relation_log()))
        .add_optional_service(Some(grpc_server.clone().ds_project()))
        .add_optional_service(Some(grpc_server.clone().ds_project_parameter()))
        .add_optional_service(Some(grpc_server.clone().ds_queue()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_datasource_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_namespace_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_process_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_project_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_resources_user()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_rule_execute_sql()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_rule_input()))
        .add_optional_service(Some(grpc_server.clone().ds_relation_udfs_user()))
        .add_optional_service(Some(grpc_server.clone().ds_resources()))
        .add_optional_service(Some(grpc_server.clone().ds_schedules()))
        .add_optional_service(Some(grpc_server.clone().ds_session()))
        .add_optional_service(Some(grpc_server.clone().ds_task_definition()))
        .add_optional_service(Some(grpc_server.clone().ds_task_definition_log()))
        .add_optional_service(Some(grpc_server.clone().ds_task_group()))
        .add_optional_service(Some(grpc_server.clone().ds_task_group_queue()))
        .add_optional_service(Some(grpc_server.clone().ds_task_instance()))
        .add_optional_service(Some(grpc_server.clone().ds_tenant()))
        .add_optional_service(Some(grpc_server.clone().ds_udfs()))
        .add_optional_service(Some(grpc_server.clone().ds_user()))
        .add_optional_service(Some(grpc_server.clone().ds_version()))
        .add_optional_service(Some(grpc_server.clone().ds_worker_group()))
        .add_optional_service(Some(grpc_server.clone().qrtz_blob_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_calendars()))
        .add_optional_service(Some(grpc_server.clone().qrtz_cron_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_fired_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_job_details()))
        .add_optional_service(Some(grpc_server.clone().qrtz_locks()))
        .add_optional_service(Some(grpc_server.clone().qrtz_paused_trigger_grps()))
        .add_optional_service(Some(grpc_server.clone().qrtz_scheduler_state()))
        .add_optional_service(Some(grpc_server.clone().qrtz_simple_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_simprop_triggers()))
        .add_optional_service(Some(grpc_server.clone().qrtz_triggers()))
}

#[allow(unused)]
async fn twiddle_service_status(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;

        if iter % 2 == 0 {
            println!("twiddle_service_status: serving");
            reporter.set_serving::<GreeterServer<MyGreeter>>().await;
        } else {
            println!("twiddle_service_status: not serving");
            reporter.set_not_serving::<GreeterServer<MyGreeter>>().await;
        };
    }
}
