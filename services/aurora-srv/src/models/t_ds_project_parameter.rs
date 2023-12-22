use anyhow::Result;
use chrono::NaiveDateTime;
use lib_proto::ds_project_parameter::ProjectParameter;
use sqlx::PgPool;
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Model {
    pub id: i32,
    pub param_name: String,
    pub param_value: String,
    pub code: i64,
    pub project_code: i64,
    pub user_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ModelPage {
    pub id: i32,
    pub param_name: String,
    pub param_value: String,
    pub code: i64,
    pub project_code: i64,
    pub user_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub count: Option<i64>,
}
impl From<ModelPage> for ProjectParameter {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            param_name: item.param_name,
            param_value: item.param_value,
            code: item.code,
            project_code: item.project_code,
            user_id: item.user_id,
            create_time: Some(item.create_time.unwrap().to_string()),
            update_time: Some(item.update_time.unwrap().to_string()),
        }
    }
}
impl From<Model> for ProjectParameter {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            param_name: item.param_name,
            param_value: item.param_value,
            code: item.code,
            project_code: item.project_code,
            user_id: item.user_id,
            create_time: Some(item.create_time.unwrap().to_string()),
            update_time: Some(item.update_time.unwrap().to_string()),
        }
    }
}

impl Model {
    pub(crate) async fn page(
        search_val: &str,
        page_num: i64,
        page_size: i64,
        project_code: i64,
        pool: &PgPool,
    ) -> Result<(Vec<ModelPage>, i64, i64, i64, i64)> {
        let search = format!("%{}%", search_val);
        let limit = page_size;
        let offset = (page_num - 1) * page_size;
        let items = sqlx::query_as!(
            ModelPage,
            r#"
            select project.*,
            coalesce (
                (select count(*) from  t_ds_project_parameter where param_name like $1 and project_code = $4),
                 0 ) "count"
            from t_ds_project_parameter as project where project.param_name like $1 and project.project_code = $4
            order by project.create_time desc
            limit $2 offset $3
            "#,
            search,
            limit,
            offset,
            project_code
        )
        .fetch_all(pool)
        .await?;

        let total = items.first().map(|x| x.count).unwrap_or(Some(0)).unwrap_or(0);
        let total_page = (total as f64 / page_size as f64).ceil() as i64;
        let items = items.into_iter().collect();
        let start = (page_num - 1) * page_size;
        let cur_page = page_num;
        Ok((items, total_page, total, start, cur_page))
    }
    pub(crate) async fn create(
        param_name: &str,
        param_value: &str,
        code: i64,
        project_code: i64,
        user_id: Option<i32>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_project_parameter (param_name,param_value,code,project_code,user_id)
            values ($1,$2,$3,$4,$5) returning  *
            "#,
            param_name,
            param_value,
            code,
            project_code,
            user_id
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn update_by_code_and_project_code(
        param_name: &str,
        param_value: &str,
        code: i64,
        project_code: i64,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(
        sqlx::query_as!(
            Self,
            r#"
            update t_ds_project_parameter set param_name = $1,param_value = $2 where code = $3 and project_code = $4 returning *
            "#,
            param_name,
            param_value,
            code,
            project_code
        ).fetch_one(pool).await?
      )
    }

    pub(crate) async fn delete_by_code_and_project_code(code: i64, project_code: i64, pool: &PgPool) -> Result<bool> {
        Ok(sqlx::query!(
            r#"
            delete from t_ds_project_parameter where code = $1 and project_code = $2
            "#,
            code,
            project_code
        )
        .execute(pool)
        .await?
        .rows_affected()
            > 0)
    }
}
