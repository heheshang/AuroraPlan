-- Add down migration script here
-- This file should undo anything in `up.sql`
drop table if exists public.qrtz_blob_triggers cascade;

drop table if exists public.qrtz_calendars cascade;

drop table if exists public.qrtz_cron_triggers cascade;

drop table if exists public.qrtz_fired_triggers cascade;

drop table if exists public.qrtz_job_details cascade;

drop table if exists public.qrtz_locks cascade;

drop table if exists public.qrtz_paused_trigger_grps cascade;

drop table if exists public.qrtz_scheduler_state cascade;

drop table if exists public.qrtz_simple_triggers cascade;

drop table if exists public.qrtz_simprop_triggers cascade;

drop table if exists public.qrtz_triggers cascade;

drop table if exists public.t_ds_access_token cascade;

drop table if exists public.t_ds_alert cascade;

drop table if exists public.t_ds_alert_plugin_instance cascade;

drop table if exists public.t_ds_alert_send_status cascade;

drop table if exists public.t_ds_alertgroup cascade;

drop table if exists public.t_ds_audit_log cascade;

drop table if exists public.t_ds_cluster cascade;

drop table if exists public.t_ds_command cascade;

drop table if exists public.t_ds_datasource cascade;

drop table if exists public.t_ds_dq_comparison_type cascade;

drop table if exists public.t_ds_dq_execute_result cascade;

drop table if exists public.t_ds_dq_rule cascade;

drop table if exists public.t_ds_dq_rule_execute_sql cascade;

drop table if exists public.t_ds_dq_rule_input_entry cascade;

drop table if exists public.t_ds_dq_task_statistics_value cascade;

drop table if exists public.t_ds_environment cascade;

drop table if exists public.t_ds_environment_worker_group_relation cascade;

drop table if exists public.t_ds_error_command cascade;

drop table if exists public.t_ds_fav_task cascade;

drop table if exists public.t_ds_k8s cascade;

drop table if exists public.t_ds_k8s_namespace cascade;

drop table if exists public.t_ds_plugin_define cascade;

drop table if exists public.t_ds_process_definition cascade;

drop table if exists public.t_ds_process_definition_log cascade;

drop table if exists public.t_ds_process_instance cascade;

drop table if exists public.t_ds_process_task_relation cascade;

drop table if exists public.t_ds_process_task_relation_log cascade;

drop table if exists public.t_ds_project cascade;

drop table if exists public.t_ds_project_parameter cascade;

drop table if exists public.t_ds_project_preference cascade;

drop table if exists public.t_ds_queue cascade;

drop table if exists public.t_ds_relation_datasource_user cascade;

drop table if exists public.t_ds_relation_namespace_user cascade;

drop table if exists public.t_ds_relation_process_instance cascade;

drop table if exists public.t_ds_relation_project_user cascade;

drop table if exists public.t_ds_relation_resources_user cascade;

drop table if exists public.t_ds_relation_rule_execute_sql cascade;

drop table if exists public.t_ds_relation_rule_input_entry cascade;

drop table if exists public.t_ds_relation_sub_workflow cascade;

drop table if exists public.t_ds_relation_udfs_user cascade;

drop table if exists public.t_ds_resources cascade;

drop table if exists public.t_ds_schedules cascade;

drop table if exists public.t_ds_session cascade;

drop table if exists public.t_ds_task_definition cascade;

drop table if exists public.t_ds_task_definition_log cascade;

drop table if exists public.t_ds_task_group cascade;

drop table if exists public.t_ds_task_group_queue cascade;

drop table if exists public.t_ds_task_instance cascade;

drop table if exists public.t_ds_tenant cascade;

drop table if exists public.t_ds_trigger_relation cascade;

drop table if exists public.t_ds_udfs cascade;

drop table if exists public.t_ds_user cascade;

drop table if exists public.t_ds_version cascade;

drop table if exists public.t_ds_worker_group cascade;


---
drop sequence if exists public.t_ds_access_token_id_sequence  cascade;

drop sequence if exists public.t_ds_alert_id_sequence  cascade;

drop sequence if exists public.t_ds_alert_plugin_instance_id_seq cascade;

drop sequence if exists public.t_ds_alert_send_status_id_seq cascade;

drop sequence if exists public.t_ds_alertgroup_id_sequence  cascade;

drop sequence if exists public.t_ds_audit_log_id_seq cascade;

drop sequence if exists public.t_ds_cluster_id_seq cascade;

drop sequence if exists public.t_ds_command_id_sequence  cascade;

drop sequence if exists public.t_ds_datasource_id_sequence  cascade;

drop sequence if exists public.t_ds_dq_comparison_type_id_seq cascade;

drop sequence if exists public.t_ds_dq_execute_result_id_seq cascade;

drop sequence if exists public.t_ds_dq_rule_execute_sql_id_seq cascade;

drop sequence if exists public.t_ds_dq_rule_id_seq cascade;

drop sequence if exists public.t_ds_dq_rule_input_entry_id_seq cascade;

drop sequence if exists public.t_ds_dq_task_statistics_value_id_seq cascade;

drop sequence if exists public.t_ds_environment_id_seq cascade;

drop sequence if exists public.t_ds_environment_worker_group_relation_id_seq cascade;

drop sequence if exists public.t_ds_fav_task_id_seq cascade;

drop sequence if exists public.t_ds_k8s_id_seq cascade;

drop sequence if exists public.t_ds_k8s_namespace_id_seq cascade;

drop sequence if exists public.t_ds_plugin_define_id_seq cascade;

drop sequence if exists public.t_ds_process_definition_id_sequence cascade;

drop sequence if exists public.t_ds_process_definition_log_id_sequence cascade;

drop sequence if exists public.t_ds_process_instance_id_sequence cascade;

drop sequence if exists public.t_ds_process_task_relation_id_sequence cascade;

drop sequence if exists public.t_ds_process_task_relation_log_id_sequence cascade;

drop sequence if exists public.t_ds_project_id_sequence cascade;

drop sequence if exists public.t_ds_project_parameter_id_sequence cascade;

drop sequence if exists public.t_ds_project_preference_id_sequence cascade;

drop sequence if exists public.t_ds_queue_id_sequence cascade;

drop sequence if exists public.t_ds_relation_datasource_user_id_sequence cascade;

drop sequence if exists public.t_ds_relation_namespace_user_id_seq cascade;

drop sequence if exists public.t_ds_relation_process_instance_id_sequence cascade;

drop sequence if exists public.t_ds_relation_project_user_id_sequence cascade;

drop sequence if exists public.t_ds_relation_resources_user_id_sequence cascade;

drop sequence if exists public.t_ds_relation_rule_execute_sql_id_seq cascade;

drop sequence if exists public.t_ds_relation_rule_input_entry_id_seq cascade;

drop sequence if exists public.t_ds_relation_sub_workflow_id_seq cascade;

drop sequence if exists public.t_ds_relation_udfs_user_id_sequence cascade;

drop sequence if exists public.t_ds_resources_id_sequence cascade;

drop sequence if exists public.t_ds_schedules_id_sequence cascade;

drop sequence if exists public.t_ds_task_definition_id_sequence cascade;

drop sequence if exists public.t_ds_task_definition_log_id_sequence cascade;

drop sequence if exists public.t_ds_task_group_id_seq cascade;

drop sequence if exists public.t_ds_task_group_queue_id_seq cascade;

drop sequence if exists public.t_ds_task_instance_id_sequence cascade;

drop sequence if exists public.t_ds_tenant_id_sequence cascade;

drop sequence if exists public.t_ds_trigger_relation_id_seq cascade;

drop sequence if exists public.t_ds_udfs_id_sequence cascade;

drop sequence if exists public.t_ds_user_id_sequence cascade;

drop sequence if exists public.t_ds_version_id_sequence cascade;

drop sequence if exists public.t_ds_worker_group_id_sequence cascade;