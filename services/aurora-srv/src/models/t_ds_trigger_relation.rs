use chrono::NaiveDateTime;
pub struct Model {
    pub id: i32,
    pub trigger_type: i32,
    pub trigger_code: i64,
    pub job_id: i64,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
