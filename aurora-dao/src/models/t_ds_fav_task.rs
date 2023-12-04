use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_fav_task)]
pub struct Model {
    pub id: i32,
    pub task_type: String,
    pub user_id: i32,
}
