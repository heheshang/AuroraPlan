use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_task_group_queue)]
pub struct Model {
    pub id: i32,
    pub task_id: Option<i32>,
    pub task_name: Option<String>,
    pub group_id: Option<i32>,
    pub process_id: Option<i32>,
    pub priority: Option<i32>,
    pub status: Option<i32>,
    pub force_start: Option<i32>,
    pub in_queue: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
