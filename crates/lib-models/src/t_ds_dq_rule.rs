use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub type_: Option<i32>,
    pub user_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
