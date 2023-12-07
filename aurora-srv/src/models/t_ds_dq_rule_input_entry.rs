use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    pub field: Option<String>,
    pub type_: Option<String>,
    pub title: Option<String>,
    pub value: Option<String>,
    pub options: Option<String>,
    pub placeholder: Option<String>,
    pub option_source_type: Option<i32>,
    pub value_type: Option<i32>,
    pub input_type: Option<i32>,
    pub is_show: Option<i16>,
    pub can_edit: Option<i16>,
    pub is_emit: Option<i16>,
    pub is_validate: Option<i16>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
