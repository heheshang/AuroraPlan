use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_locks)]
#[diesel(primary_key(sched_name, lock_name))]
pub struct Model {
    pub sched_name: String,
    pub lock_name: String,
}
