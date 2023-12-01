use chrono::NaiveDateTime;
use diesel::prelude::*;
#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::t_ds_environment_relation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EnvironmentRelation {
    #[diesel(column_name = "id")]
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub worker_groups: Option<Vec<Option<String>>>,
}
