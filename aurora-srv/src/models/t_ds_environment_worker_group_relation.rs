use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub environment_code: i64,
    pub worker_group: String,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
