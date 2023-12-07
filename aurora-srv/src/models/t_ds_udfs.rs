use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub func_name: String,
    pub class_name: String,
    pub type_: i32,
    pub arg_types: Option<String>,
    pub database: Option<String>,
    pub description: Option<String>,
    pub resource_id: i32,
    pub resource_name: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
