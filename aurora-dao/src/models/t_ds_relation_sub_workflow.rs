use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_relation_sub_workflow)]
pub struct Model {
    pub id: i32,
    pub parent_workflow_instance_id: i64,
    pub parent_task_code: i64,
    pub sub_workflow_instance_id: i64,
}
