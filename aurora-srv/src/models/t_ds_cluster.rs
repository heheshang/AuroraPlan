use anyhow::Result;
use aurora_common::utils::code_generate_utils::gen_code;
use chrono::NaiveDateTime;
use proto::ds_cluster::DsCluster;

pub struct Model {
    pub id: i32,
    // #[sea_orm(unique)]
    pub code: i64,
    // #[sea_orm(unique)]
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
pub struct ModelPage {
    pub id: i32,
    // #[sea_orm(unique)]
    pub code: i64,
    // #[sea_orm(unique)]
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub count: Option<i64>,
}
impl From<ModelPage> for Model {
    fn from(model: ModelPage) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            config: model.config,
            description: model.description,
            operator: model.operator,
            create_time: model.create_time,
            update_time: model.update_time,
        }
    }
}

impl From<Model> for DsCluster {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            config: model.config,
            description: model.description,
            operator: model.operator,
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
            select *,count(*) over() as count from t_ds_cluster where name like $1 order by id desc limit $2 offset $3;
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
        name: Option<String>,
        config: Option<String>,
        description: Option<String>,
        operator: Option<i32>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        let code = gen_code().unwrap_or_default();
        Ok(sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_cluster (name,config,description,operator,code) values ($1,$2,$3,$4,$5) returning *;
            "#,
            name,
            config,
            description,
            operator,
            code
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _update(
        code: i64,
        name: Option<String>,
        config: Option<String>,
        description: Option<String>,
        operator: Option<i32>,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                update t_ds_cluster set name=$1,config=$2,description=$3,operator=$4 where code=$5 returning *;
                "#,
            name,
            config,
            description,
            operator,
            code
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _find_by_name(name: String, pool: &sqlx::PgPool) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_cluster where name=$1;
            "#,
            name
        )
        .fetch_optional(pool)
        .await?)
    }
    ///delete by id
    pub(crate) async fn _delete(code: i64, pool: &sqlx::PgPool) -> Result<u64> {
        Ok(sqlx::query!(
            r#"
            delete from t_ds_cluster where code=$1;
            "#,
            code
        )
        .execute(pool)
        .await?
        .rows_affected())
    }
}
