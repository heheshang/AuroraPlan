use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub alias: Option<String>,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub type_: Option<i32>,
    pub size: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub pid: Option<i32>,
    pub full_name: Option<String>,
    pub is_directory: Option<bool>,
}
