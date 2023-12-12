use anyhow::Result;
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
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn _create(
        sched_name: String,
        trigger_name: String,
        trigger_group: String,
        job_name: String,
        job_group: String,
        description: Option<String>,
        next_fire_time: Option<i64>,
        prev_fire_time: Option<i64>,
        priority: Option<i32>,
        trigger_state: String,
        trigger_type: String,
        start_time: i64,
        end_time: Option<i64>,
        calendar_name: Option<String>,
        misfire_instr: Option<i16>,
        job_data: Option<Vec<u8>>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        let model = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO qrtz_triggers (
                sched_name,
                trigger_name,
                trigger_group,
                job_name,
                job_group,
                description,
                next_fire_time,
                prev_fire_time,
                priority,
                trigger_state,
                trigger_type,
                start_time,
                end_time,
                calendar_name,
                misfire_instr,
                job_data
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16) returning *;
            "#,
            sched_name,
            trigger_name,
            trigger_group,
            job_name,
            job_group,
            description,
            next_fire_time,
            prev_fire_time,
            priority,
            trigger_state,
            trigger_type,
            start_time,
            end_time,
            calendar_name,
            misfire_instr,
            job_data
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
}
