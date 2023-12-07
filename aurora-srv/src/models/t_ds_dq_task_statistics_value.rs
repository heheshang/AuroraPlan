use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub process_definition_id: i32,
    pub task_instance_id: Option<i32>,
    pub rule_id: i32,
    pub unique_code: String,
    pub statistics_name: Option<String>,
    pub statistics_value: Option<f64>,
    pub data_time: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
