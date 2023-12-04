use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_task_group)]
pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group_size: i32,
    pub project_code: Option<i64>,
    pub use_size: Option<i32>,
    pub user_id: Option<i32>,
    pub status: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
