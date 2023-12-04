use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_alertgroup)]
#[diesel(primary_key(id))]

pub struct Model {
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    // #[diesel(unique)]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
