use std::process::Command;

use prost_build::Config;

use proto_builder_trait::prost::BuilderAttributes;

fn main() {
    // Config::default()
    //     .out_dir("src/pb")

    //     .with_serde(
    //         &["ds_user.DsUser", "ds_session.DsSession"],
    //         true,
    //         true,
    //         Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
    //         // None,
    //     )
    //     .with_derive_builder(
    //         &["ds_user.DsUser", "ds_session.DsSession"],
    //         Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    //     )
    //     // .with_sqlx_type(&["ds_user.DsUser", "ds_session.DsSession"], None)
    //     // .with_strum(
    //     //     &["ds_user.DsUser", "ds_session.DsSession"],
    //     //     Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
    //     // )
    //     //   .with_serde_as(
    //     //         "ds_user.DsUser",
    //     //         &[(
    //     //             &["update_time", "create_time"],
    //     //             r#"#[serde_as(as = "chrono::DateTime<chrono::Utc>")]"#,
    //     //         )],
    //     //     )
    //     // .with_field_attributes(&["ds_user.DsUser.create_time", "ds_user.DsUser.update_time"], &[
    //     //     "#[derive(Copy)]",
    //     // ])
    //     .compile_protos(&["protos/ds_session.proto","protos/ds_user.proto"], &["protos"])
    //     .unwrap();

    // let c = Config::default()
    //     .out_dir("src/pb")
    //     .with_serde(
    //         &["todo.Todo", "todo.TodoStatus"],
    //         true,
    //         true,
    //         Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
    //     )
    //     .with_serde_as("todo.Todo", &[(
    //         &["status", "created_at"],
    //         r#"#[serde_as(as = "DisplayFromStr")]"#,
    //     )])
    //     .with_derive_builder(
    //         &["todo.Todo"],
    //         Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    //     )
    //     // .with_sqlx_type(&["todo.TodoStatus"], None)
    //     .with_strum(
    //         &["todo.TodoStatus"],
    //         Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
    //     );
    // .with_field_attributes(&["todo.Todo.created_at", "todo.Todo.updated_at"], &[
    //     "#[derive(Copy)]",
    // ])
    //     .service_generator(ServiceGenerator::new())
    //      .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"]);

    // tonic_build::configure()
    // .compile_with_config(c, &["fixtures/protos/todo.proto"], &["fixtures/protos"])

    //     .unwrap();

    // let _ = prost_build::compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"]);
    // let _ = prost_build::compile_protos(&["protos/ds_session.proto"], &["protos"]);
    // let _ = prost_build::compile_protos(&["protos/ds_user.proto"], &["protos"]);

    let mut config = Config::default();
    config.with_serde(
        &[
            "ds_access_token.DsAccessToken",
            "ds_alert_plugin_instance.DsAlertPluginInstance",
            "ds_alert_send_status.DsAlertSendStatus",
            "ds_alert.DsAlert",
            "ds_alertgroup.DsAlertGroup",
            "ds_audit_log.DsAuditLog",
            "ds_command.DsCommand",
            "ds_datasource.DsDatasource",
            "ds_dq_comparison_type.DsDqComparisonType",
            "ds_dq_execute_result.DsDqExecuteResult",
            "ds_dq_rule_execute_sql.DsDqRuleExecuteSql",
            "ds_dq_rule_input_entry.DsDqRuleInputEntry",
            "ds_dq_rule.DsDqRule",
            "ds_dq_task_statistics_value.DsDqTaskStatisticsValue",
            "ds_environment_worker_group_relation.DsEnvironmentWorkerGroupRelation",
            "ds_environment.DsEnvironment",
            "ds_error_command.DsErrorCommand",
            "ds_k8s_namespace.DsK8sNamespace",
            "ds_k8s.DsK8s",
            "ds_plugin_define.DsPluginDefine",
            "ds_process_definition_log.DsProcessDefinitionLog",
            "ds_process_definition.DsProcessDefinition",
            "ds_process_instance.DsProcessInstance",
            "ds_process_task_relation_log.DsProcessTaskRelationLog",
            "ds_process_task_relation.DsProcessTaskRelation",
            "ds_project.DsProject",
            "ds_project_parameter.ProjectParameter",
            "ds_queue.DsQueue",
            "ds_relation_datasource_user.DsRelationDatasourceUser",
            "ds_relation_namespace_user.DsRelationNamespaceUser",
            "ds_relation_process_instance.DsRelationProcessInstance",
            "ds_relation_project_user.DsRelationProjectUser",
            "ds_relation_resources_user.DsRelationResourcesUser",
            "ds_relation_rule_execute_sql.DsRelationRuleExecuteSql",
            "ds_relation_rule_input.DsRelationRuleInput",
            "ds_relation_udfs_user.DsRelationUdfsUser",
            "ds_resources.DsResource",
            "ds_schedules.DsSchedules",
            "ds_session.DsSession",
            "ds_task_definition_log.DsTaskDefinitionLog",
            "ds_task_definition.DsTaskDefinition",
            "ds_task_group_queue.DsTaskGroupQueue",
            "ds_task_group.DsTaskGroup",
            "ds_task_instance.DsTaskInstance",
            "ds_tenant.DsTenant",
            "ds_udfs.DsUdfs",
            "ds_user.DsUser",
            "ds_user.Flag",
            "ds_user.UserType",
            "ds_version.DsVersion",
            "ds_worker_group.DsWorkerGroup",
            "qrtz_blob_triggers.QrtzBlobTrigger",
            "qrtz_calendars.QrtzCalendar",
            "qrtz_cron_triggers.QrtzCronTriggers",
            "qrtz_fired_triggers.QrtzFiredTriggers",
            "qrtz_job_details.QrtzJobDetails",
            "qrtz_locks.QrtzLocks",
            "qrtz_paused_trigger_grps.QrtzPausedTriggerGrps",
            "qrtz_scheduler_state.QrtzSchedulerState",
            "qrtz_simple_triggers.QrtzSimpleTrigger",
            "qrtz_simprop_triggers.QrtzSimpropTrigger",
            "qrtz_triggers.QrtzTrigger",
        ],
        true,
        true,
        Some(&[r#"#[serde(rename_all = "snake_case")]"#]),
        // None,
    );
    config.with_serde_as(
        "ds_user.DsUser",
        &[(
            &["create_time", "update_time"],
            r#"#[serde_as(as = "DisplayFromStr")]"#,
        )],
    );
    config.with_derive_builder(
        &["ds_user.DsUser", "ds_session.DsSession"],
        Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    );

    tonic_build::configure()
        .out_dir("src/pb")
        .compile_with_config(
            config,
            &[
                "protos/googleapis/google/helloworld/helloworld.proto",
                "protos/googleapis/google/ds/v1/qrtz_blob_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_calendars.proto",
                "protos/googleapis/google/ds/v1/qrtz_cron_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_fired_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_job_details.proto",
                "protos/googleapis/google/ds/v1/qrtz_locks.proto",
                "protos/googleapis/google/ds/v1/qrtz_paused_trigger_grps.proto",
                "protos/googleapis/google/ds/v1/qrtz_scheduler_state.proto",
                "protos/googleapis/google/ds/v1/qrtz_simple_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_simprop_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_triggers.proto",
                "protos/googleapis/google/ds/v1/t_ds_access_token.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert_plugin_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert_send_status.proto",
                "protos/googleapis/google/ds/v1/t_ds_alertgroup.proto",
                "protos/googleapis/google/ds/v1/t_ds_audit_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_command.proto",
                "protos/googleapis/google/ds/v1/t_ds_datasource.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_comparison_type.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_execute_result.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule_execute_sql.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule_input_entry.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_task_statistics_value.proto",
                "protos/googleapis/google/ds/v1/t_ds_environment.proto",
                "protos/googleapis/google/ds/v1/t_ds_environment_worker_group_relation.proto",
                "protos/googleapis/google/ds/v1/t_ds_error_command.proto",
                "protos/googleapis/google/ds/v1/t_ds_k8s.proto",
                "protos/googleapis/google/ds/v1/t_ds_k8s_namespace.proto",
                "protos/googleapis/google/ds/v1/t_ds_plugin_define.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_definition.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_definition_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_task_relation.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_task_relation_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_project.proto",
                "protos/googleapis/google/ds/v1/t_ds_project_parameter.proto",
                "protos/googleapis/google/ds/v1/t_ds_queue.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_datasource_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_namespace_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_process_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_project_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_resources_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_rule_execute_sql.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_rule_input.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_udfs_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_resources.proto",
                "protos/googleapis/google/ds/v1/t_ds_schedules.proto",
                "protos/googleapis/google/ds/v1/t_ds_session.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_definition.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_definition_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_group.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_group_queue.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_tenant.proto",
                "protos/googleapis/google/ds/v1/t_ds_udfs.proto",
                "protos/googleapis/google/ds/v1/t_ds_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_version.proto",
                "protos/googleapis/google/ds/v1/t_ds_worker_group.proto",
            ],
            &["protos/googleapis"],
        )
        .unwrap();
    //    tonic_build::configure()
    //       .build_server(false)
    //       .out_dir("src/pb")
    //       .compile(&["protos/googleapis/google/pubsub/v1/pubsub.proto"], &[
    //           "protos/googleapis",
    //       ])
    //       .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();
}
