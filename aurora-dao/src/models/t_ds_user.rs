use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_user)]
pub struct User {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
}

// #[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
// #[sea_orm(rs_type = "i32", db_type = "Integer)]
// #[allow(non_camel_case_types)]
// pub enum Usertype {
//     #[sea_orm(num_value = 1)]
//     ADMIN_USER,
//     #[sea_orm(num_value = 2)]
//     GENERAL_USER,
// }
