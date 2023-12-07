use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub version: i32,
    pub description: Option<String>,
    pub project_code: Option<i64>,
    pub release_state: Option<i32>,
    pub user_id: Option<i32>,
    pub global_params: Option<String>,
    pub locations: Option<String>,
    pub warning_group_id: Option<i32>,
    pub flag: Option<i32>,
    pub timeout: Option<i32>,
    pub execution_type: Option<i32>,
    pub operator: Option<i32>,
    pub operate_time: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
