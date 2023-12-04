use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_dq_rule_execute_sql)]
pub struct Model {
    pub id: i32,
    pub index: Option<i32>,
    pub sql: Option<String>,
    pub table_alias: Option<String>,
    pub type_: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_error_output_sql: Option<bool>,
}
