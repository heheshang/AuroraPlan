use chrono::NaiveDateTime;

pub struct Model {
    pub id: String,
    pub user_id: Option<i32>,
    pub ip: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
}
