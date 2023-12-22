use anyhow::Result;
use sqlx::PgPool;
pub struct Model {
    pub sched_name: String,
    pub entry_id: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub instance_name: String,
    pub fired_time: i64,
    pub sched_time: i64,
    pub priority: i32,
    pub state: String,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub is_nonconcurrent: Option<bool>,
    pub requests_recovery: Option<bool>,
}
impl Model {
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn _create(
        sched_name: String,
        entry_id: String,
        trigger_name: String,
        trigger_group: String,
        instance_name: String,
        fired_time: i64,
        sched_time: i64,
        priority: i32,
        state: String,
        job_name: Option<String>,
        job_group: Option<String>,
        is_nonconcurrent: Option<bool>,
        requests_recovery: Option<bool>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into qrtz_fired_triggers \
             (sched_name,entry_id,trigger_name,trigger_group,instance_name,fired_time,sched_time,priority,state,\
             job_name,job_group,is_nonconcurrent,requests_recovery ) values \
             ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) returning *",
            sched_name,
            entry_id,
            trigger_name,
            trigger_group,
            instance_name,
            fired_time,
            sched_time,
            priority,
            state,
            job_name,
            job_group,
            is_nonconcurrent,
            requests_recovery
        )
        .fetch_one(pool)
        .await?)
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn _update(
        sched_name: String,
        entry_id: String,
        trigger_name: String,
        trigger_group: String,
        instance_name: String,
        fired_time: i64,
        sched_time: i64,
        priority: i32,
        state: String,
        job_name: Option<String>,
        job_group: Option<String>,
        is_nonconcurrent: Option<bool>,
        requests_recovery: Option<bool>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
           update qrtz_fired_triggers set trigger_name=$1,trigger_group=$2,instance_name=$3,fired_time=$4,sched_time=$5,priority=$6,state=$7,job_name=$8,job_group=$9,is_nonconcurrent=$10,requests_recovery=$11 where sched_name=$12 and entry_id=$13 returning *
            "#,
            trigger_name,
            trigger_group,
            instance_name,
            fired_time,
            sched_time,
            priority,
            state,
            job_name,
            job_group,
            is_nonconcurrent,
            requests_recovery,
            sched_name,
            entry_id
        )
        .fetch_one(pool)
        .await?)
    }
}
