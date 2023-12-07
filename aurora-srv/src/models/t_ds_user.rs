use anyhow::Result;
use chrono::NaiveDateTime;
use proto::ds_user::DsUser;
use sqlx::{query_as, PgPool};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
}

impl User {
    pub async fn find_by_id(_id: i32, pool: &PgPool) -> Result<Self> {
        let user = sqlx::query_as!(User, "select * from t_ds_user where id = $1", _id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
    pub async fn find_by_name(name: &str, pool: &PgPool) -> Result<Option<Self>> {
        let user = sqlx::query_as!(Self, r#"select * from t_ds_user where user_name = $1"#, name)
            .fetch_one(pool)
            .await?;
        Ok(Some(user))
    }
    pub async fn update(
        _id: i32,
        _mail: Option<String>,
        _phone: Option<String>,
        _user_type: Option<i32>,
        pool: &PgPool,
    ) -> Result<usize> {
        let row = sqlx::query!(
            r#"update t_ds_user set email = $1, phone = $2, user_type = $3 where id = $4"#,
            _mail,
            _phone,
            _user_type,
            _id
        )
        .execute(pool)
        .await?;
        Ok(row.rows_affected() as usize)
    }
    pub async fn query_user_by_name_password(_name: &str, _password: &str, pool: &PgPool) -> Result<Option<Self>> {
        let user = query_as!(
            Self,
            r#"select * from t_ds_user where user_name = $1 and user_password = $2"#,
            _name,
            _password
        )
        .fetch_one(pool)
        .await?;
        Ok(Some(user))
    }
}
impl From<User> for DsUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            user_name: user.user_name,
            user_password: user.user_password,
            user_type: user.user_type,
            email: user.email,
            phone: user.phone,
            tenant_id: user.tenant_id,
            create_time: user.create_time.map(|v| v.to_string()),
            update_time: user.update_time.map(|v| v.to_string()),
            queue: user.queue,
            state: user.state,
            time_zone: user.time_zone,
        }
    }
}
