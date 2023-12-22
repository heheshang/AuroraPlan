use anyhow::Result;
pub struct Model {
    pub sched_name: String,
    pub instance_name: String,
    pub last_checkin_time: i64,
    pub checkin_interval: i64,
}

impl Model {
    #[allow(dead_code)]
    pub async fn get(pool: &sqlx::PgPool, sched_name: &str, instance_name: &str) -> Result<Self> {
        let result = sqlx::query_as!(
            Self,
            "SELECT * FROM qrtz_scheduler_state WHERE sched_name=$1 AND instance_name=$2",
            sched_name,
            instance_name
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }
    #[allow(dead_code)]
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Self>> {
        let result = sqlx::query_as!(Self, "SELECT * FROM qrtz_scheduler_state")
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}
