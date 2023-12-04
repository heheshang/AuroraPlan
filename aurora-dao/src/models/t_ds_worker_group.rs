use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_worker_group)]
#[diesel(primary_key(id))]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub addr_list: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub other_params_json: Option<String>,
}
