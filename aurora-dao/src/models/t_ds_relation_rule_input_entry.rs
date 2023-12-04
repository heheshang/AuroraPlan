use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_relation_rule_input_entry)]
pub struct Model {
    pub id: i32,
    pub rule_id: Option<i32>,
    pub rule_input_entry_id: Option<i32>,
    pub values_map: Option<String>,
    pub index: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
