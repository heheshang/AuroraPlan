use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub rule_id: Option<i32>,
    pub rule_input_entry_id: Option<i32>,
    pub values_map: Option<String>,
    pub index: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
