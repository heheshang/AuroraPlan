use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_process_task_relation_log)]
pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub project_code: Option<i64>,
    pub process_definition_code: Option<i64>,
    pub process_definition_version: Option<i32>,
    pub pre_task_code: Option<i64>,
    pub pre_task_version: Option<i32>,
    pub post_task_code: Option<i64>,
    pub post_task_version: Option<i32>,
    pub condition_type: Option<i32>,
    pub condition_params: Option<String>,
    pub operator: Option<i32>,
    pub operate_time: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
