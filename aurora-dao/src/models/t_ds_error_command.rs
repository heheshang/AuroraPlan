use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_error_command)]
pub struct Model {
    pub id: i32,
    pub command_type: Option<i32>,
    pub process_definition_code: i64,
    pub command_param: Option<String>,
    pub task_depend_type: Option<i32>,
    pub failure_strategy: Option<i32>,
    pub warning_type: Option<i32>,
    pub warning_group_id: Option<i32>,
    pub schedule_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub executor_id: Option<i32>,
    pub update_time: Option<NaiveDateTime>,
    pub process_instance_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub tenant_code: Option<String>,
    pub environment_code: Option<i64>,
    pub dry_run: Option<i32>,
    pub message: Option<String>,
    pub process_instance_id: Option<i32>,
    pub process_definition_version: Option<i32>,
    pub test_flag: Option<i32>,
}
