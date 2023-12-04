use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_dq_comparison_type)]
pub struct Model {
    pub id: i32,
    pub type_: String,
    pub execute_sql: Option<String>,
    pub output_table: Option<String>,
    pub name: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_inner_source: Option<bool>,
}
