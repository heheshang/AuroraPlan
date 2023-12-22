use anyhow::Result;
use sqlx::PgPool;
pub struct Model {
    pub sched_name: String,
    pub calendar_name: String,
    pub calendar: Vec<u8>,
}

impl Model {
    pub async fn _create(sched_name: String, calendar_name: String, calendar: String, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into qrtz_calendars (sched_name,calendar_name,calendar ) values ($1,$2,$3) returning *",
            sched_name,
            calendar_name,
            calendar.as_bytes()
        )
        .fetch_one(pool)
        .await?)
    }
    pub async fn _update(sched_name: String, calendar_name: String, calendar: String, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
           update qrtz_calendars set calendar_name=$1,calendar=$2 where sched_name=$3 returning *
            "#,
            calendar_name,
            calendar.as_bytes(),
            sched_name
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn _find_by_name(sched_name: String, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from qrtz_calendars where sched_name=$1 
            "#,
            sched_name
        )
        .fetch_one(pool)
        .await?)
    }
}
