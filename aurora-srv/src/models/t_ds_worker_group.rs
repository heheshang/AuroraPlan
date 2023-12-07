use chrono::NaiveDateTime;

pub struct Model {
    pub id: i64,
    pub name: String,
    pub addr_list: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub other_params_json: Option<String>,
}
