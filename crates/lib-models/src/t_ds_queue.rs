use chrono::NaiveDateTime;

use anyhow::Result;
use lib_proto::ds_queue::DsQueue;
use sqlx::PgPool;
#[derive(Debug, Clone, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
impl From<Model> for DsQueue {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            queue_name: item.queue_name,
            queue: item.queue,
            create_time: Some(item.create_time.unwrap().to_string()),
            update_time: Some(item.update_time.unwrap().to_string()),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ModelPage {
    pub id: i32,
    pub queue_name: Option<String>,
    pub queue: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub count: Option<i64>,
}
impl From<ModelPage> for Model {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            queue_name: item.queue_name,
            queue: item.queue,
            create_time: item.create_time,
            update_time: item.update_time,
        }
    }
}

impl Model {
    pub async fn page(
        search_val: &str,
        page_num: i64,
        page_size: i64,
        pool: &PgPool,
    ) -> Result<(Vec<Self>, i64, i64, i64, i64)> {
        let search = format!("%{}%", search_val);
        let limit = page_size;
        let offset = (page_num - 1) * page_size;

        let items = sqlx::query_as!(
            ModelPage,
            r#"
            select * ,
            coalesce (
                 (select count(*) from  t_ds_queue where queue_name like $1), 0 ) "count" 
                 from t_ds_queue where queue_name like $1 order by create_time desc 
                 limit $2 
                 offset $3
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

    pub async fn queue_name_count(queue_name: String, pool: &PgPool) -> Result<i64> {
        let count = sqlx::query!(
            r#"
         select count(*) as count from t_ds_queue where queue_name=$1
         "#,
            queue_name
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or_default();
        Ok(count)
    }
    pub async fn queue_value_count(queue_value: String, pool: &PgPool) -> Result<i64> {
        let count = sqlx::query!(
            r#"
         select count(*) as count from t_ds_queue where queue=$1
         "#,
            queue_value
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or_default();
        Ok(count)
    }
    pub async fn queue_name_count_extra(queue_name: &str, id: i32, pool: &PgPool) -> Result<i64> {
        let count = sqlx::query!(
            r#"
         select count(*) as count from t_ds_queue where queue_name=$1 and id<>$2
         "#,
            queue_name,
            id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or_default();
        Ok(count)
    }

    pub async fn queue_value_count_extra(queue: &str, id: i32, pool: &PgPool) -> Result<i64> {
        let count = sqlx::query!(
            r#"
         select count(*) as count from t_ds_queue where queue=$1 and id<>$2
         "#,
            queue,
            id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or_default();
        Ok(count)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         select * from t_ds_queue where id=$1
         "#,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn find_by_queue_name(queue_name: String, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         select * from t_ds_queue where queue_name=$1
         "#,
            queue_name
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn find_by_queue(queue: String, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         select * from t_ds_queue where queue=$1
         "#,
            queue
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }

    pub async fn create(queue_name: String, queue: String, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         insert into t_ds_queue(queue_name,queue) values($1,$2) returning *
         "#,
            queue_name,
            queue
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn update(id: i32, queue_name: Option<String>, queue: Option<String>, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         update t_ds_queue set queue_name=$1,queue=$2 where id=$3 returning *
         "#,
            queue_name,
            queue,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool> {
        let count = sqlx::query!(
            r#"
         delete from t_ds_queue where id=$1
         "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();
        Ok(count > 0)
    }
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let items = sqlx::query_as!(
            Model,
            r#"
         select * from t_ds_queue order by create_time desc
         "#
        )
        .fetch_all(pool)
        .await?;
        Ok(items)
    }
}
