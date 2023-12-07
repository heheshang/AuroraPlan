use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub index: Option<i32>,
    pub sql: Option<String>,
    pub table_alias: Option<String>,
    pub type_: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_error_output_sql: Option<bool>,
}
