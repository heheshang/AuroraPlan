-- Add up migration script here

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, update_time TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT sqlx_manage_update_time('users');

CREATE OR REPLACE FUNCTION sqlx_manage_update_time(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_update_time BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE sqlx_set_update_time()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION sqlx_set_update_time() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.update_time IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.update_time := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;



select
'SELECT sqlx_manage_update_time(public.'|| table_name||');'
FROM information_schema.columns
WHERE column_name = 'update_time';

select 'alter table public.' || table_name || ' alter column create_time set default current_timestamp;'
FROM information_schema.columns
WHERE column_name = 'create_time';
select 'alter table public.' || table_name || ' alter column update_time set default current_timestamp;'
FROM information_schema.columns
WHERE column_name = 'update_time';


alter table public.t_ds_access_token alter column create_time set default current_timestamp;
alter table public.t_ds_alert alter column create_time set default current_timestamp;
alter table public.t_ds_alert_plugin_instance alter column create_time set default current_timestamp;
alter table public.t_ds_alert_send_status alter column create_time set default current_timestamp;
alter table public.t_ds_alertgroup alter column create_time set default current_timestamp;
alter table public.t_ds_cluster alter column create_time set default current_timestamp;
alter table public.t_ds_datasource alter column create_time set default current_timestamp;
alter table public.t_ds_dq_comparison_type alter column create_time set default current_timestamp;
alter table public.t_ds_dq_execute_result alter column create_time set default current_timestamp;
alter table public.t_ds_dq_rule alter column create_time set default current_timestamp;
alter table public.t_ds_dq_rule_execute_sql alter column create_time set default current_timestamp;
alter table public.t_ds_dq_rule_input_entry alter column create_time set default current_timestamp;
alter table public.t_ds_dq_task_statistics_value alter column create_time set default current_timestamp;
alter table public.t_ds_environment_worker_group_relation alter column create_time set default current_timestamp;
alter table public.t_ds_k8s alter column create_time set default current_timestamp;
alter table public.t_ds_k8s_namespace alter column create_time set default current_timestamp;
alter table public.t_ds_plugin_define alter column create_time set default current_timestamp;
alter table public.t_ds_environment alter column create_time set default current_timestamp;
alter table public.t_ds_process_definition alter column create_time set default current_timestamp;
alter table public.t_ds_process_definition_log alter column create_time set default current_timestamp;
alter table public.t_ds_process_task_relation alter column create_time set default current_timestamp;
alter table public.t_ds_process_task_relation_log alter column create_time set default current_timestamp;
alter table public.t_ds_project alter column create_time set default current_timestamp;
alter table public.t_ds_project_parameter alter column create_time set default current_timestamp;
alter table public.t_ds_project_preference alter column create_time set default current_timestamp;
alter table public.t_ds_queue alter column create_time set default current_timestamp;
alter table public.t_ds_relation_datasource_user alter column create_time set default current_timestamp;
alter table public.t_ds_relation_namespace_user alter column create_time set default current_timestamp;
alter table public.t_ds_relation_project_user alter column create_time set default current_timestamp;
alter table public.t_ds_relation_resources_user alter column create_time set default current_timestamp;
alter table public.t_ds_relation_rule_execute_sql alter column create_time set default current_timestamp;
alter table public.t_ds_relation_rule_input_entry alter column create_time set default current_timestamp;
alter table public.t_ds_relation_udfs_user alter column create_time set default current_timestamp;
alter table public.t_ds_resources alter column create_time set default current_timestamp;
alter table public.t_ds_schedules alter column create_time set default current_timestamp;
alter table public.t_ds_task_definition alter column create_time set default current_timestamp;
alter table public.t_ds_task_definition_log alter column create_time set default current_timestamp;
alter table public.t_ds_task_group alter column create_time set default current_timestamp;
alter table public.t_ds_task_group_queue alter column create_time set default current_timestamp;
alter table public.t_ds_trigger_relation alter column create_time set default current_timestamp;
alter table public.t_ds_udfs alter column create_time set default current_timestamp;
alter table public.t_ds_user alter column create_time set default current_timestamp;
alter table public.t_ds_worker_group alter column create_time set default current_timestamp;
alter table public.t_ds_tenant alter column create_time set default current_timestamp;
alter table public.v_ds_environment alter column create_time set default current_timestamp;
alter table public.t_ds_environment_relation alter column create_time set default current_timestamp;


alter table public.t_ds_access_token alter column update_time set default current_timestamp;
alter table public.t_ds_alert alter column update_time set default current_timestamp;
alter table public.t_ds_alert_plugin_instance alter column update_time set default current_timestamp;
alter table public.t_ds_alertgroup alter column update_time set default current_timestamp;
alter table public.t_ds_cluster alter column update_time set default current_timestamp;
alter table public.t_ds_command alter column update_time set default current_timestamp;
alter table public.t_ds_datasource alter column update_time set default current_timestamp;
alter table public.t_ds_dq_comparison_type alter column update_time set default current_timestamp;
alter table public.t_ds_dq_execute_result alter column update_time set default current_timestamp;
alter table public.t_ds_dq_rule alter column update_time set default current_timestamp;
alter table public.t_ds_dq_rule_execute_sql alter column update_time set default current_timestamp;
alter table public.t_ds_dq_rule_input_entry alter column update_time set default current_timestamp;
alter table public.t_ds_dq_task_statistics_value alter column update_time set default current_timestamp;
alter table public.t_ds_environment_worker_group_relation alter column update_time set default current_timestamp;
alter table public.t_ds_error_command alter column update_time set default current_timestamp;
alter table public.t_ds_k8s alter column update_time set default current_timestamp;
alter table public.t_ds_k8s_namespace alter column update_time set default current_timestamp;
alter table public.t_ds_plugin_define alter column update_time set default current_timestamp;
alter table public.t_ds_environment alter column update_time set default current_timestamp;
alter table public.t_ds_process_definition alter column update_time set default current_timestamp;
alter table public.t_ds_process_definition_log alter column update_time set default current_timestamp;
alter table public.t_ds_process_instance alter column update_time set default current_timestamp;
alter table public.t_ds_process_task_relation alter column update_time set default current_timestamp;
alter table public.t_ds_process_task_relation_log alter column update_time set default current_timestamp;
alter table public.t_ds_project alter column update_time set default current_timestamp;
alter table public.t_ds_project_parameter alter column update_time set default current_timestamp;
alter table public.t_ds_project_preference alter column update_time set default current_timestamp;
alter table public.t_ds_queue alter column update_time set default current_timestamp;
alter table public.t_ds_relation_datasource_user alter column update_time set default current_timestamp;
alter table public.t_ds_relation_namespace_user alter column update_time set default current_timestamp;
alter table public.t_ds_relation_project_user alter column update_time set default current_timestamp;
alter table public.t_ds_relation_resources_user alter column update_time set default current_timestamp;
alter table public.t_ds_relation_rule_execute_sql alter column update_time set default current_timestamp;
alter table public.t_ds_relation_rule_input_entry alter column update_time set default current_timestamp;
alter table public.t_ds_relation_udfs_user alter column update_time set default current_timestamp;
alter table public.t_ds_resources alter column update_time set default current_timestamp;
alter table public.t_ds_schedules alter column update_time set default current_timestamp;
alter table public.t_ds_task_definition alter column update_time set default current_timestamp;
alter table public.t_ds_task_definition_log alter column update_time set default current_timestamp;
alter table public.t_ds_task_group alter column update_time set default current_timestamp;
alter table public.t_ds_task_group_queue alter column update_time set default current_timestamp;
alter table public.t_ds_trigger_relation alter column update_time set default current_timestamp;
alter table public.t_ds_udfs alter column update_time set default current_timestamp;
alter table public.t_ds_user alter column update_time set default current_timestamp;
alter table public.t_ds_worker_group alter column update_time set default current_timestamp;
alter table public.t_ds_tenant alter column update_time set default current_timestamp;
alter table public.v_ds_environment alter column update_time set default current_timestamp;
alter table public.t_ds_environment_relation alter column update_time set default current_timestamp;



SELECT sqlx_manage_update_time('public.t_ds_access_token');
SELECT sqlx_manage_update_time('public.t_ds_alert');
SELECT sqlx_manage_update_time('public.t_ds_alert_plugin_instance');
SELECT sqlx_manage_update_time('public.t_ds_alertgroup');
SELECT sqlx_manage_update_time('public.t_ds_cluster');
SELECT sqlx_manage_update_time('public.t_ds_command');
SELECT sqlx_manage_update_time('public.t_ds_datasource');
SELECT sqlx_manage_update_time('public.t_ds_dq_comparison_type');
SELECT sqlx_manage_update_time('public.t_ds_dq_execute_result');
SELECT sqlx_manage_update_time('public.t_ds_dq_rule');
SELECT sqlx_manage_update_time('public.t_ds_dq_rule_execute_sql');
SELECT sqlx_manage_update_time('public.t_ds_dq_rule_input_entry');
SELECT sqlx_manage_update_time('public.t_ds_dq_task_statistics_value');
SELECT sqlx_manage_update_time('public.t_ds_environment_worker_group_relation');
SELECT sqlx_manage_update_time('public.t_ds_error_command');
SELECT sqlx_manage_update_time('public.t_ds_k8s');
SELECT sqlx_manage_update_time('public.t_ds_k8s_namespace');
SELECT sqlx_manage_update_time('public.t_ds_plugin_define');
SELECT sqlx_manage_update_time('public.t_ds_environment');
SELECT sqlx_manage_update_time('public.t_ds_process_definition');
SELECT sqlx_manage_update_time('public.t_ds_process_definition_log');
SELECT sqlx_manage_update_time('public.t_ds_process_instance');
SELECT sqlx_manage_update_time('public.t_ds_process_task_relation');
SELECT sqlx_manage_update_time('public.t_ds_process_task_relation_log');
SELECT sqlx_manage_update_time('public.t_ds_project');
SELECT sqlx_manage_update_time('public.t_ds_project_parameter');
SELECT sqlx_manage_update_time('public.t_ds_project_preference');
SELECT sqlx_manage_update_time('public.t_ds_queue');
SELECT sqlx_manage_update_time('public.t_ds_relation_datasource_user');
SELECT sqlx_manage_update_time('public.t_ds_relation_namespace_user');
SELECT sqlx_manage_update_time('public.t_ds_relation_project_user');
SELECT sqlx_manage_update_time('public.t_ds_relation_resources_user');
SELECT sqlx_manage_update_time('public.t_ds_relation_rule_execute_sql');
SELECT sqlx_manage_update_time('public.t_ds_relation_rule_input_entry');
SELECT sqlx_manage_update_time('public.t_ds_relation_udfs_user');
SELECT sqlx_manage_update_time('public.t_ds_resources');
SELECT sqlx_manage_update_time('public.t_ds_schedules');
SELECT sqlx_manage_update_time('public.t_ds_task_definition');
SELECT sqlx_manage_update_time('public.t_ds_task_definition_log');
SELECT sqlx_manage_update_time('public.t_ds_task_group');
SELECT sqlx_manage_update_time('public.t_ds_task_group_queue');
SELECT sqlx_manage_update_time('public.t_ds_trigger_relation');
SELECT sqlx_manage_update_time('public.t_ds_udfs');
SELECT sqlx_manage_update_time('public.t_ds_user');
SELECT sqlx_manage_update_time('public.t_ds_worker_group');
SELECT sqlx_manage_update_time('public.t_ds_tenant');
SELECT sqlx_manage_update_time('public.t_ds_environment_relation');
