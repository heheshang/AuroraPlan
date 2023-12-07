use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub alert_id: i32,
    pub alert_plugin_instance_id: i32,
    pub send_status: Option<i32>,
    pub log: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}
