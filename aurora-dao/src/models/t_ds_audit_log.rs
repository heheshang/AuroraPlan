use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_audit_log)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub resource_type: i32,
    pub operation: i32,
    pub time: Option<NaiveDateTime>,
    pub resource_id: i32,
}
