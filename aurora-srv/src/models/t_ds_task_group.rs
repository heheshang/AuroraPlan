use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group_size: i32,
    pub project_code: Option<i64>,
    pub use_size: Option<i32>,
    pub user_id: Option<i32>,
    pub status: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
