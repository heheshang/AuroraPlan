use chrono::NaiveDateTime;

use anyhow::Result;
use lib_proto::ds_worker_group::DsWorkerGroup;
use sqlx::PgPool;
#[derive(Debug, Clone, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub addr_list: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub other_params_json: Option<String>,
}

impl From<Model> for DsWorkerGroup {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            name: item.name,
            addr_list: item.addr_list,
            create_time: Some(item.create_time.unwrap().to_string()),
            update_time: Some(item.update_time.unwrap().to_string()),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ModelPage {
    pub id: i64,
    pub name: String,
    pub addr_list: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub other_params_json: Option<String>,
    pub count: Option<i64>,
}
impl From<ModelPage> for Model {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            name: item.name,
            addr_list: item.addr_list,
            create_time: item.create_time,
            update_time: item.update_time,
            description: item.description,
            other_params_json: item.other_params_json,
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
                 (select count(*) from  t_ds_worker_group where name like $1), 0 ) "count" 
                 from t_ds_worker_group where name like $1 order by create_time desc 
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

    // pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<Self> {
    //     let model = sqlx::query_as!(
    //         Model,
    //         r#"
    //      select * from t_ds_worker_group where id=$1
    //      "#,
    //         id
    //     )
    //     .fetch_one(pool)
    //     .await?;
    //     Ok(model)
    // }

    pub async fn create(name: String, addr: Option<String>, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         insert into t_ds_worker_group(name,addr_list) values($1,$2) returning *
         "#,
            name,
            addr
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn update(id: i64, name: Option<String>, pool: &PgPool) -> Result<Self> {
        let model = sqlx::query_as!(
            Model,
            r#"
         update t_ds_worker_group set name=$1 where id=$2 returning *
         "#,
            name,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(model)
    }
    pub async fn delete(id: i64, pool: &PgPool) -> Result<bool> {
        let count = sqlx::query!(
            r#"
         delete from t_ds_worker_group where id=$1
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
         select * from t_ds_worker_group order by create_time desc
         "#
        )
        .fetch_all(pool)
        .await?;
        Ok(items)
    }
}
