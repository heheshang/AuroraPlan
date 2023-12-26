use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub struct Model {
    pub id: i32,
    pub command_type: Option<i32>,
    pub process_definition_code: i64,
    pub command_param: Option<String>,
    pub task_depend_type: Option<i32>,
    pub failure_strategy: Option<i32>,
    pub warning_type: Option<i32>,
    pub warning_group_id: Option<i32>,
    pub schedule_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub executor_id: Option<i32>,
    pub update_time: Option<NaiveDateTime>,
    pub process_instance_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub tenant_code: Option<String>,
    pub environment_code: Option<i64>,
    pub dry_run: Option<i32>,
    pub process_instance_id: Option<i32>,
    pub process_definition_version: Option<i32>,
    pub test_flag: Option<i32>,
}

impl Model {
    pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Model,
            r#"
            SELECT * FROM t_ds_command WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
}
