use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub udf_id: Option<i32>,
    pub perm: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
