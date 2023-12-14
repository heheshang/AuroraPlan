use anyhow::Result;
use chrono::NaiveDateTime;
use proto::ds_alert_plugin_instance::DsAlertPluginInstance;
pub struct Model {
    pub id: i32,
    pub plugin_define_id: i32,
    pub plugin_instance_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub instance_name: Option<String>,
}
pub struct ModelPage {
    pub id: i32,
    pub plugin_define_id: i32,
    pub plugin_instance_params: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub instance_name: Option<String>,
    pub count: Option<i64>,
}
impl From<ModelPage> for Model {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            plugin_define_id: item.plugin_define_id,
            plugin_instance_params: item.plugin_instance_params,
            instance_name: item.instance_name,
            create_time: item.create_time,
            update_time: item.update_time,
        }
    }
}

impl From<ModelPage> for DsAlertPluginInstance {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            plugin_define_id: item.plugin_define_id,
            plugin_instance_params: item.plugin_instance_params,
            instance_name: item.instance_name,
            create_time: item.create_time.map(|t| t.to_string()),
            update_time: item.update_time.map(|t| t.to_string()),
        }
    }
}
impl From<Model> for DsAlertPluginInstance {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            plugin_define_id: item.plugin_define_id,
            plugin_instance_params: item.plugin_instance_params,
            instance_name: item.instance_name,
            create_time: item.create_time.map(|t| t.to_string()),
            update_time: item.update_time.map(|t| t.to_string()),
        }
    }
}
impl Model {
    pub(crate) async fn page(
        search_val: &str,
        page_num: i64,
        page_size: i64,
        pool: &sqlx::PgPool,
    ) -> Result<(Vec<Self>, i64, i64, i64, i64)> {
        let search = format!("%{}%", search_val);
        let limit = page_size;
        let offset = (page_num - 1) * page_size;

        let items = sqlx::query_as!(
            ModelPage,
            r#"
            select count(1) over() as count, * from t_ds_alert_plugin_instance where instance_name like $1 order by id desc limit $2 offset $3
                "#,
            search,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        let total = items.first().map(|x| x.count).unwrap_or(Some(0)).unwrap_or(0);
        let total_page = (total as f64 / page_size as f64).ceil() as i64;
        let items = items.into_iter().map(Self::from).collect::<Vec<Self>>();
        let start = (page_num - 1) * page_size;
        let cur_page = page_num;
        Ok((items, total_page, total, start, cur_page))
    }
    pub(crate) async fn find_by_instance_name(instance_name: String, pool: &sqlx::PgPool) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_alert_plugin_instance where instance_name=$1 
            "#,
            instance_name
        )
        .fetch_optional(pool)
        .await?)
    }
    pub(crate) async fn _create(
        plugin_define_id: i32,
        plugin_instance_params: Option<String>,
        instance_name: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into t_ds_alert_plugin_instance (plugin_define_id,plugin_instance_params,instance_name ) values 
             ($1,$2,$3) returning *",
            plugin_define_id,
            plugin_instance_params,
            instance_name
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _update(
        id: i32,
        plugin_instance_params: Option<String>,
        instance_name: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "update t_ds_alert_plugin_instance set plugin_instance_params=$2,instance_name=$3 where id=$1 returning *",
            id,
            plugin_instance_params,
            instance_name
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
    pub(crate) async fn _delete(id: i32, pool: &sqlx::PgPool) -> Result<u64> {
        Ok(sqlx::query!(
            r#"
            delete from t_ds_alert_plugin_instance where id=$1 
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected())
    }
    pub(crate) async fn all(pool: &sqlx::PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(Self, "select * from t_ds_alert_plugin_instance")
            .fetch_all(pool)
            .await?)
    }
}
