use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_session)]
pub struct Model {
    pub id: String,
    pub user_id: Option<i32>,
    pub ip: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
}
