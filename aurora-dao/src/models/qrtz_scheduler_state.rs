use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_scheduler_state)]
#[diesel(primary_key(sched_name, instance_name))]
pub struct Model {
    pub sched_name: String,
    pub instance_name: String,
    pub last_checkin_time: i64,
    pub checkin_interval: i64,
}
