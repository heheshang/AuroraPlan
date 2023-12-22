pub struct Model {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub cron_expression: String,
    pub time_zone_id: Option<String>,
}
