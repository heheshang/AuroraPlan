use anyhow::Result;
use chrono::NaiveDateTime;
use lib_proto::ds_plugin_define as dto;
use sqlx::PgPool;
pub struct Model {
    pub id: i32,
    pub plugin_name: String,
    pub plugin_type: String,
    pub plugin_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
impl From<Model> for dto::DsPluginDefine {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            plugin_name: m.plugin_name,
            plugin_type: m.plugin_type,
            plugin_params: m.plugin_params,
            create_time: m.create_time.map(|t| t.to_string()),
            update_time: m.update_time.map(|t| t.to_string()),
        }
    }
}
impl Model {
    pub(crate) async fn query_by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM t_ds_plugin_define WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(Self {
            id: row.id,
            plugin_name: row.plugin_name,
            plugin_type: row.plugin_type,
            plugin_params: row.plugin_params,
            create_time: row.create_time,
            update_time: row.update_time,
        })
    }
    pub(crate) async fn query_by_type(plugin_type: &str, pool: &PgPool) -> Result<Vec<Self>> {
        let rows = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM t_ds_plugin_define WHERE plugin_type = $1
            "#,
            plugin_type
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}
