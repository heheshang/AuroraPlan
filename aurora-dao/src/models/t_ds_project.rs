use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_project)]
pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub code: i64,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub flag: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    // #[sea_orm(ignore)]
    // pub user_name: String,
}
