use anyhow::Result;
use chrono::NaiveDateTime;
use lib_proto::ds_alertgroup::DsAlertGroup;
pub struct Model {
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    // #[diesel(unique)]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
pub struct ModelPage {
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    // #[diesel(unique)]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub count: Option<i64>,
}

impl From<ModelPage> for Model {
    fn from(model: ModelPage) -> Self {
        Self {
            id: model.id,
            alert_instance_ids: model.alert_instance_ids,
            create_user_id: model.create_user_id,
            group_name: model.group_name,
            description: model.description,
            create_time: model.create_time,
            update_time: model.update_time,
        }
    }
}

impl From<Model> for DsAlertGroup {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            alert_instance_ids: model.alert_instance_ids,
            create_user_id: model.create_user_id,
            group_name: model.group_name,
            description: model.description,
            create_time: model.create_time.map(|x| x.to_string()),
            update_time: model.update_time.map(|x| x.to_string()),
        }
    }
}
impl Model {
    /// page function for t_ds_alertgroup
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
            select count(1) over() as count, * from t_ds_alertgroup where group_name like $1 order by id desc limit $2 offset $3
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

    pub(crate) async fn _create(
        alert_instance_ids: Option<String>,
        create_user_id: Option<i32>,
        group_name: Option<String>,
        description: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into t_ds_alertgroup (alert_instance_ids,create_user_id,group_name,description ) values \
             ($1,$2,$3,$4) returning *",
            alert_instance_ids,
            create_user_id,
            group_name,
            description
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _update(
        id: i32,
        alert_instance_ids: Option<String>,
        create_user_id: Option<i32>,
        group_name: Option<String>,
        description: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            update t_ds_alertgroup set alert_instance_ids=$1,create_user_id=$2,group_name=$3,description=$4 where id=$5 returning *;
            "#,
            alert_instance_ids,
            create_user_id,
            group_name,
            description,
            id
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _find_by_name(group_name: String, pool: &sqlx::PgPool) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_alertgroup where group_name=$1 
            "#,
            group_name
        )
        .fetch_optional(pool)
        .await?)
    }
    ///delete by id
    pub(crate) async fn _delete(id: i32, pool: &sqlx::PgPool) -> Result<u64> {
        Ok(sqlx::query!(
            r#"
            delete from t_ds_alertgroup where id=$1
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected())
    }
}
