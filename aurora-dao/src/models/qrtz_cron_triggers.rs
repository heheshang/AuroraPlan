use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Default)]
#[diesel(table_name = crate::schema::qrtz_cron_triggers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Model {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub cron_expression: String,
    pub time_zone_id: Option<String>,
}
