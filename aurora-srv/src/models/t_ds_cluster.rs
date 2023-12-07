use chrono::NaiveDateTime;

pub struct Model {
    pub id: i32,
    // #[sea_orm(unique)]
    pub code: i64,
    // #[sea_orm(unique)]
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
