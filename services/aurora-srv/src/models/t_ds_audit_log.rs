use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub resource_type: i32,
    pub operation: i32,
    pub time: Option<NaiveDateTime>,
    pub resource_id: i32,
}
