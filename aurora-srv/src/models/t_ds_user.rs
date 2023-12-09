use anyhow::Result;
use chrono::NaiveDateTime;
use proto::ds_user::{DsUser, DsUserPage};
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

#[derive(Debug, Clone, Default)]
pub struct UserPage {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_type: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: Option<i32>,
    pub tenant_code: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub queue: Option<String>,
    pub state: Option<i32>,
    pub time_zone: Option<String>,
    pub count: Option<i64>,
}

impl From<UserPage> for User {
    fn from(item: UserPage) -> Self {
        Self {
            id: item.id,
            user_name: item.user_name,
            user_password: item.user_password,
            user_type: item.user_type,
            email: item.email,
            phone: item.phone,
            tenant_id: item.tenant_id,
            create_time: item.create_time,
            update_time: item.update_time,
            queue: item.queue,
            state: item.state,
            time_zone: item.time_zone,
        }
    }
}

impl From<UserPage> for DsUserPage {
    fn from(item: UserPage) -> Self {
        Self {
            id: item.id,
            user_name: item.user_name,
            user_password: item.user_password,
            user_type: item.user_type,
            email: item.email,
            phone: item.phone,
            tenant_id: item.tenant_id,
            tenant_code: item.tenant_code,
            create_time: item.create_time.map(|v| v.to_string()),
            update_time: item.update_time.map(|v| v.to_string()),
            queue: item.queue,
            state: item.state,
            time_zone: item.time_zone,
        }
    }
}

impl User {
    pub(crate) async fn page(
        search_val: &str,
        page_num: i64,
        page_size: i64,
        pool: &PgPool,
    ) -> Result<(Vec<UserPage>, i64, i64, i64, i64)> {
        let search = format!("%{}%", search_val);
        let limit = page_size;
        let offset = (page_num - 1) * page_size;

        let items = sqlx::query_as!(
            UserPage,
            r#"
            select t_ds_user.* ,
            coalesce (
                 (select count(*) from  t_ds_user where user_name like $1), 0 ) "count" ,
            t_ds_tenant.tenant_code
            from  t_ds_user left join t_ds_tenant on t_ds_user.tenant_id = t_ds_tenant.id
            where user_name like $1
            order by t_ds_tenant.create_time desc
            limit $2 offset $3
            "#,
            search,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = items.first().map(|v| v.count.unwrap_or(0)).unwrap_or(0);
        // let items = items.into_iter().map(|v| v.into()).collect::<Vec<_>>();
        let total_page = (total as f64 / page_size as f64).ceil() as i64;
        let start = (page_num - 1) * page_size;
        let cur_page = page_num;
        Ok((items, total_page, total, start, cur_page))
    }

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
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn create(
        user_name: Option<String>,
        user_password: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        tenant_id: Option<i32>,
        queue: Option<String>,
        state: Option<i32>,
        pool: &PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            User,
            r#"
            insert into t_ds_user (user_name,user_password,email,phone,tenant_id,queue,state)
            values ($1,$2,$3,$4,$5,$6,$7)
            returning *
            "#,
            user_name,
            user_password,
            email,
            phone,
            tenant_id,
            queue,
            state,
        )
        .fetch_one(pool)
        .await?)
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        id: i32,
        user_name: Option<String>,
        tenant_id: Option<i32>,
        email: Option<String>,
        queue: Option<String>,
        phone: Option<String>,
        state: Option<i32>,

        pool: &PgPool,
    ) -> Result<usize> {
        let row = sqlx::query!(
            r#"
            update t_ds_user set
            user_name = $1,
            tenant_id = $2,
            email = $3,
            queue = $4,
            phone = $5,
            state = $6
            where id = $7
            "#,
            user_name,
            tenant_id,
            email,
            queue,
            phone,
            state,
            id
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
    pub(crate) async fn query_user_by_name(_name: &str, pool: &PgPool) -> Result<Option<Self>> {
        let user = query_as!(Self, r#"select * from t_ds_user where user_name = $1"#, _name)
            .fetch_optional(pool)
            .await?;
        Ok(user)
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
