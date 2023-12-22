use anyhow::Result;
use chrono::NaiveDateTime;
use lib_proto::ds_session::DsSession;
use sqlx::PgPool;
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Model {
    pub id: String,
    pub user_id: Option<i32>,
    pub ip: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
}

impl From<Model> for DsSession {
    fn from(item: Model) -> Self {
        // let parse_from_str = DateTime::parse_from_str;
        //         parse_from_str(&ds_session.last_login_time.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap_or(current_time),

        Self {
            id: item.id,
            user_id: item.user_id.unwrap_or_default(),
            ip: item.ip,
            last_login_time: Some(item.last_login_time.unwrap().to_string()),
        }
    }
}

impl Model {
    pub(crate) async fn create(id: String, user_id: i32, ip: Option<String>, pool: &PgPool) -> Result<Self> {
        let current_time = chrono::prelude::Local::now().naive_local();
        Ok(sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_session (id,user_id,ip,last_login_time)
            values ($1,$2,$3,$4) returning *
            "#,
            id,
            user_id,
            ip,
            current_time
        )
        .fetch_one(pool)
        .await?)
    }

    pub(crate) async fn update(id: String, user_id: i32, ip: Option<String>, pool: &PgPool) -> Result<Self> {
        let current_time = chrono::prelude::Local::now().naive_local();
        Ok(sqlx::query_as!(
            Self,
            r#"
            update t_ds_session set user_id=$1,ip=$2,last_login_time=$3
            where id=$4 returning *
            "#,
            user_id,
            ip,
            current_time,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub(crate) async fn delete(id: String, pool: &PgPool) -> Result<bool> {
        Ok(sqlx::query!(
            r#"
            delete from t_ds_session where id=$1
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected()
            > 0)
    }

    pub(crate) async fn find_by_id(id: String, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_session where id=$1
            "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn find_by_user_id(user_id: i32, pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_session where user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?)
    }
}
