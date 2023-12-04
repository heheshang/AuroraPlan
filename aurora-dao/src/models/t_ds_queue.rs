use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_queue)]
pub struct Model {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}