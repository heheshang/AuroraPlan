use anyhow::Result;
use chrono::NaiveDateTime;
use lib_proto::ds_tenant::DsTenant;
use sqlx::PgPool;

pub struct Model {
    pub id: i32,
    pub tenant_code: Option<String>,
    pub description: Option<String>,
    pub queue_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
pub struct ModelPage {
    pub id: i32,
    pub tenant_code: Option<String>,
    pub description: Option<String>,
    pub queue_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub count: Option<i64>,
}
pub struct ModelForm {
    pub id: Option<i32>,
    pub tenant_code: Option<String>,
    pub description: Option<String>,
    pub queue_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
impl From<ModelPage> for Model {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            tenant_code: item.tenant_code,
            description: item.description,
            queue_id: item.queue_id,
            create_time: item.create_time,
            update_time: item.update_time,
        }
    }
}

impl Model {
    pub(crate) async fn page(
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
                 (select count(*) from  t_ds_tenant where tenant_code like $1), 0 ) "count" 
                 from t_ds_tenant where tenant_code like $1 order by create_time desc 
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

    pub(crate) async fn create(
        code: Option<String>,
        desc: Option<String>,
        queue: Option<i32>,
        pool: &PgPool,
    ) -> Result<Self> {
        let row = sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_tenant (tenant_code, description, queue_id) values ($1, $2, $3) returning *
            "#,
            code,
            desc,
            queue,
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub(crate) async fn find_by_code(code: &str, pool: &PgPool) -> Result<Option<Self>> {
        let res = sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_tenant where tenant_code = $1
            "#,
            code
        )
        .fetch_optional(pool)
        .await?;
        Ok(res)
    }

    pub(crate) async fn update(_id: i32, desc: String, queue: i32, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            update t_ds_tenant set description = $1, queue_id = $2 where id = $3 returning *
            "#,
            desc,
            queue,
            _id
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn delete(_id: i32, pool: &PgPool) -> Result<usize> {
        Ok(sqlx::query_as!(Self, r#"delete from t_ds_tenant where id = $1"#, _id)
            .execute(pool)
            .await?
            .rows_affected() as usize)
    }
    pub(crate) async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let res = sqlx::query_as!(Self, r#"select * from t_ds_tenant"#).fetch_all(pool).await?;
        Ok(res)
    }
}

impl From<Model> for DsTenant {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            tenant_code: item.tenant_code,
            description: item.description,
            queue_id: item.queue_id,
            create_time: item.create_time.map(|t| t.to_string()),
            update_time: item.update_time.map(|t| t.to_string()),
        }
    }
}
