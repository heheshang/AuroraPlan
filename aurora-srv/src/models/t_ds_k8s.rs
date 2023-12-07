use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub k8s_name: Option<String>,
    pub k8s_config: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
