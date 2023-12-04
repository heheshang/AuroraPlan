use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_project_preference)]
pub struct Model {
    pub id: i32,
    pub code: i64,
    pub project_code: i64,
    pub preferences: String,
    pub user_id: Option<i32>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
