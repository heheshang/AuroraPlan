pub struct Model {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub job_name: String,
    pub job_group: String,
    pub description: Option<String>,
    pub next_fire_time: Option<i64>,
    pub prev_fire_time: Option<i64>,
    pub priority: Option<i32>,
    pub trigger_state: String,
    pub trigger_type: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub calendar_name: Option<String>,
    pub misfire_instr: Option<i16>,
    pub job_data: Option<Vec<u8>>,
}
impl Model {
    pub fn create() -> Self {
        todo!()
    }
}