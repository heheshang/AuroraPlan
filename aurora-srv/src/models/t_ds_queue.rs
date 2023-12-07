use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
