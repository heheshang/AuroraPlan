use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_udfs)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub func_name: String,
    pub class_name: String,
    pub type_: i32,
    pub arg_types: Option<String>,
    pub database: Option<String>,
    pub description: Option<String>,
    pub resource_id: i32,
    pub resource_name: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
