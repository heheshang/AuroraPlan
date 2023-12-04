use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_calendars)]
pub struct Model {
    pub sched_name: String,
    pub calendar_name: String,
    pub calendar: Vec<u8>,
}
