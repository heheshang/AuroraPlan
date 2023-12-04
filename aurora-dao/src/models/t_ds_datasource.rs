use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_datasource)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub note: Option<String>,
    pub type_: i32,
    pub user_id: i32,
    pub connection_params: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
