use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub code: i64,
    pub project_code: i64,
    pub preferences: String,
    pub user_id: Option<i32>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
