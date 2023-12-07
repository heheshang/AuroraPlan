use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub process_definition_id: Option<i32>,
    pub process_instance_id: Option<i32>,
    pub task_instance_id: Option<i32>,
    pub rule_type: Option<i32>,
    pub rule_name: Option<String>,
    pub statistics_value: Option<f64>,
    pub comparison_value: Option<f64>,
    pub check_type: Option<i32>,
    pub threshold: Option<f64>,
    pub operator: Option<i32>,
    pub failure_strategy: Option<i32>,
    pub state: Option<i32>,
    pub user_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub comparison_type: Option<i32>,
    pub error_output_path: Option<String>,
}
