use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub version: i32,
    pub description: Option<String>,
    pub project_code: Option<i64>,
    pub user_id: Option<i32>,
    pub task_type: Option<String>,
    pub task_execute_type: Option<i32>,
    pub task_params: Option<String>,
    pub flag: Option<i32>,
    pub is_cache: Option<i32>,
    pub task_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub environment_code: Option<i64>,
    pub fail_retry_times: Option<i32>,
    pub fail_retry_interval: Option<i32>,
    pub timeout_flag: Option<i32>,
    pub timeout_notify_strategy: Option<i32>,
    pub timeout: Option<i32>,
    pub delay_time: Option<i32>,
    pub resource_ids: Option<String>,
    pub operator: Option<i32>,
    pub task_group_id: Option<i32>,
    pub task_group_priority: Option<i32>,
    pub operate_time: Option<NaiveDateTime>,
    pub cpu_quota: i32,
    pub memory_max: i32,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
