use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_simple_triggers)]
#[diesel(primary_key(sched_name, trigger_name, trigger_group))]
pub struct Model {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub repeat_count: i64,
    pub repeat_interval: i64,
    pub times_triggered: i64,
}
