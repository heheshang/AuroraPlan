use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub plugin_define_id: i32,
    pub plugin_instance_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub instance_name: Option<String>,
}
