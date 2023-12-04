use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_relation_process_instance)]
pub struct Model {
    pub id: i32,
    pub parent_process_instance_id: Option<i32>,
    pub parent_task_instance_id: Option<i32>,
    pub process_instance_id: Option<i32>,
}
