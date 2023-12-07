use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub user_id: Option<i32>,
    pub token: Option<String>,
    pub expire_time: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
