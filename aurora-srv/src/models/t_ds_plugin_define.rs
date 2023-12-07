use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub plugin_name: String,
    pub plugin_type: String,
    pub plugin_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
