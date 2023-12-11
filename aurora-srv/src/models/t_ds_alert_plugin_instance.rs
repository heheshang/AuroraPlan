use anyhow::Result;
use chrono::NaiveDateTime;
pub struct Model {
    pub id: i32,
    pub plugin_define_id: i32,
    pub plugin_instance_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub instance_name: Option<String>,
}
impl Model {
    pub(crate) async fn _create(
        plugin_define_id: i32,
        plugin_instance_params: String,
        instance_name: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into t_ds_alert_plugin_instance (plugin_define_id,plugin_instance_params,instance_name ) values \
             ($1,$2,$3) returning *",
            plugin_define_id,
            plugin_instance_params,
            instance_name
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _update(
        plugin_define_id: i32,
        plugin_instance_params: String,
        instance_name: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
           update t_ds_alert_plugin_instance set plugin_instance_params=$1,instance_name=$2 where plugin_define_id=$3 returning *
            "#,
            plugin_instance_params,
            instance_name,
            plugin_define_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub(crate) async fn _find_by_name(plugin_define_id: i32, pool: &sqlx::PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_alert_plugin_instance where plugin_define_id=$1 
            "#,
            plugin_define_id
        )
        .fetch_one(pool)
        .await?)
    }
}
