use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(sched_name, entry_id))]
#[diesel(table_name = crate::schema::qrtz_fired_triggers)]
pub struct Model {
    pub sched_name: String,
    pub entry_id: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub instance_name: String,
    pub fired_time: i64,
    pub sched_time: i64,
    pub priority: i32,
    pub state: String,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub is_nonconcurrent: Option<bool>,
    pub requests_recovery: Option<bool>,
}
