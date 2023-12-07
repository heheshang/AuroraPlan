use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub name: String,
    pub note: Option<String>,
    pub type_: i32,
    pub user_id: i32,
    pub connection_params: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
