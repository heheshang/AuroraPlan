use chrono::NaiveDateTime;
use sqlx::PgPool;

use anyhow::Result;
use proto::ds_project::{DsProject, DsProjectListRes};
// alter table public.t_ds_tenant
// alter column create_time set default CURRENT_TIMESTAMP;
//     CREATE TRIGGER update_updated_at
// BEFORE UPDATE ON my_table
// FOR EACH ROW
// EXECUTE FUNCTION update_updated_at();

pub struct Model {
    pub id: i32,
    pub name: Option<String>,
    pub code: i64,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub flag: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

pub struct ModelPage {
    pub id: i32,
    pub name: Option<String>,
    pub code: i64,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub flag: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub user_name: Option<String>,
    pub count: Option<i64>,
}
impl From<ModelPage> for Model {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            name: item.name,
            code: item.code,
            description: item.description,
            user_id: item.user_id,
            flag: item.flag,
            create_time: item.create_time,
            update_time: item.update_time,
        }
    }
}
impl From<ModelPage> for DsProjectListRes {
    fn from(item: ModelPage) -> Self {
        Self {
            id: item.id,
            name: item.name,
            code: item.code,
            description: item.description,
            user_id: item.user_id,
            flag: item.flag,
            create_time: Some(item.create_time.unwrap().to_string()),
            update_time: Some(item.update_time.unwrap().to_string()),
            user_name: item.user_name.unwrap_or_default(),
        }
    }
}
impl From<Model> for DsProject {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            name: item.name,
            code: item.code,
            description: item.description,
            user_id: item.user_id,
            flag: item.flag,
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
                (select count(*) from  t_ds_project where name like $1),
                 0 ) "count",
            u.user_name  "user_name"
            from t_ds_project as project
            inner join t_ds_user as u on project.user_id = u.id and project.name like $1
            order by project.create_time desc
            limit $2 offset $3
            "#,
            search,
            limit,
            offset,
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
        code: i64,
        name: Option<String>,
        desc: Option<String>,
        user_id: Option<i32>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"insert into t_ds_project (code,name,description,user_id) values ($1,$2,$3,$4)  returning *"#,
            code,
            name,
            desc,
            user_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub(crate) async fn update(
        name: Option<String>,
        desc: Option<String>,
        user_id: Option<i32>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"update t_ds_project set name=$1,description=$2,user_id=$3   returning *"#,
            name,
            desc,
            user_id
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn delete(id: i32, pool: &PgPool) -> Result<bool> {
        Ok(sqlx::query!(r#"delete from t_ds_project where id=$1"#, id)
            .execute(pool)
            .await?
            .rows_affected()
            == 1)
    }
}
