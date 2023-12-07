use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub rule_id: Option<i32>,
    pub execute_sql_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
