use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EnvironmentRelation {
    pub id: Option<i32>,
    pub code: Option<i64>,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub worker_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EnvironmentRelationPage {
    pub id: Option<i32>,
    pub code: Option<i64>,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub worker_groups: Option<Vec<String>>,
    pub count: Option<i64>,
}

impl From<EnvironmentRelationPage> for EnvironmentRelation {
    fn from(item: EnvironmentRelationPage) -> Self {
        Self {
            id: item.id,
            code: item.code,
            name: item.name,
            config: item.config,
            description: item.description,
            operator: item.operator,
            create_time: item.create_time,
            update_time: item.update_time,
            worker_groups: item.worker_groups,
        }
    }
}

impl EnvironmentRelation {
    pub async fn find_by_code(_code: i64, pool: &PgPool) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"
            select
              *
            from  v_ds_environment t1
            where t1.code = $1
            "#,
            _code
        )
        .fetch_one(pool)
        .await?;
        todo!()
    }

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
            EnvironmentRelationPage,
            r#"
            select * ,
            coalesce (
                 (select count(*) from  v_ds_environment where name like $1), 0 ) 
            "count" 
                 from v_ds_environment where name like $1 order by create_time desc 
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
        Ok((items, total_page, total, start, cur_page)) // Ok(res.unwrap_or_default())
    }
}
