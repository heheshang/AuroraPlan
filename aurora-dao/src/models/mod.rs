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
pub mod t_ds_access_token;
pub mod t_ds_alert;
pub mod t_ds_alert_plugin_instance;
pub mod t_ds_alert_send_status;
pub mod t_ds_alertgroup;
pub mod t_ds_audit_log;
pub mod t_ds_cluster;
pub mod t_ds_command;
pub mod t_ds_datasource;
pub mod t_ds_dq_comparison_type;
pub mod t_ds_dq_execute_result;
pub mod t_ds_dq_rule;
pub mod t_ds_dq_rule_execute_sql;
pub mod t_ds_dq_rule_input_entry;
pub mod t_ds_dq_task_statistics_value;
pub mod t_ds_environment;
pub mod t_ds_environment_relation;
pub mod t_ds_environment_worker_group_relation;
pub mod t_ds_error_command;
pub mod t_ds_fav_task;
pub mod t_ds_k8s;
pub mod t_ds_k8s_namespace;
pub mod t_ds_plugin_define;
pub mod t_ds_process_definition;
pub mod t_ds_process_definition_log;
pub mod t_ds_process_instance;
pub mod t_ds_process_task_relation;
pub mod t_ds_process_task_relation_log;
pub mod t_ds_project;
pub mod t_ds_project_parameter;
pub mod t_ds_project_preference;
pub mod t_ds_queue;
pub mod t_ds_relation_datasource_user;
pub mod t_ds_relation_namespace_user;
pub mod t_ds_relation_process_instance;
pub mod t_ds_relation_project_user;
pub mod t_ds_relation_resources_user;
pub mod t_ds_relation_rule_execute_sql;
pub mod t_ds_relation_rule_input_entry;
pub mod t_ds_relation_sub_workflow;
pub mod t_ds_relation_udfs_user;
pub mod t_ds_resources;
pub mod t_ds_schedules;
pub mod t_ds_session;
pub mod t_ds_task_definition;
pub mod t_ds_task_definition_log;
pub mod t_ds_task_group;
pub mod t_ds_task_group_queue;
pub mod t_ds_task_instance;
pub mod t_ds_tenant;
pub mod t_ds_trigger_relation;
pub mod t_ds_udfs;
pub mod t_ds_user;
pub mod t_ds_version;
pub mod t_ds_worker_group;

pub type DPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
