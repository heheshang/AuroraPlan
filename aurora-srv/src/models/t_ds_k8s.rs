use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub struct Model {
    pub id: i32,
    pub k8s_name: Option<String>,
    pub k8s_config: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

impl Model {
    pub(crate) async fn _create(k8s_name: String, k8s_config: String, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_k8s (k8s_name,k8s_config) values ($1,$2) returning *
            "#,
            k8s_name,
            k8s_config
        )
        .fetch_one(pool)
        .await?)
    }
}
