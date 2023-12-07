pub struct Model {
    pub id: i32,
    pub parent_process_instance_id: Option<i32>,
    pub parent_task_instance_id: Option<i32>,
    pub process_instance_id: Option<i32>,
}
