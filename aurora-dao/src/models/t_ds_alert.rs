use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_alert)]
pub struct Model {
    pub id: i32,
    pub title: Option<String>,
    pub sign: String,
    pub content: Option<String>,
    pub alert_status: Option<i32>,
    pub warning_type: Option<i32>,
    pub log: Option<String>,
    pub alertgroup_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub project_code: Option<i64>,
    pub process_definition_code: Option<i64>,
    pub process_instance_id: Option<i32>,
    pub alert_type: Option<i32>,
}
