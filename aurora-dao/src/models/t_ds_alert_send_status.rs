use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_alert_send_status)]
pub struct Model {
    pub id: i32,
    pub alert_id: i32,
    pub alert_plugin_instance_id: i32,
    pub send_status: Option<i32>,
    pub log: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}
