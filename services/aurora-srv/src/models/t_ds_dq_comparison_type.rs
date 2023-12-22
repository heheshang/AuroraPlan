use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub type_: String,
    pub execute_sql: Option<String>,
    pub output_table: Option<String>,
    pub name: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_inner_source: Option<bool>,
}
