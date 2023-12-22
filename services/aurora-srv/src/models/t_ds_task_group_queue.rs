use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub task_id: Option<i32>,
    pub task_name: Option<String>,
    pub group_id: Option<i32>,
    pub process_id: Option<i32>,
    pub priority: Option<i32>,
    pub status: Option<i32>,
    pub force_start: Option<i32>,
    pub in_queue: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
