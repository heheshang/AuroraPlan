use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_schedules)]
pub struct Model {
    pub id: i32,
    pub process_definition_code: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub timezone_id: Option<String>,
    pub crontab: String,
    pub failure_strategy: i32,
    pub user_id: i32,
    pub release_state: i32,
    pub warning_type: i32,
    pub warning_group_id: Option<i32>,
    pub process_instance_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub tenant_code: Option<String>,
    pub environment_code: Option<i64>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
