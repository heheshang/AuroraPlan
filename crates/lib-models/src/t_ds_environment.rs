use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;
pub struct Model {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

pub struct MyInt {
    pub id: i32,
}
impl Model {
    pub async fn env_name_exist(_name: &str, pool: &PgPool) -> Result<bool> {
        Ok(sqlx::query_as!(
            MyInt,
            r#"
            select id from t_ds_environment where name = $1
            "#,
            _name
        )
        .fetch_optional(pool)
        .await?
        .is_some())
    }

    async fn insert(
        _name: &str,
        _config: &str,
        _description: Option<String>,
        _operator: i32,
        _worker_groups: Vec<String>,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Self> {
        let _code = lib_common::utils::code_generate_utils::gen_code().unwrap_or_default();
        let res = sqlx::query_as!(
            Self,
            r#"
            insert into t_ds_environment (code,name,config,description,operator)
            values ($1,$2,$3,$4,$5) returning *
            "#,
            _code,
            _name,
            _config,
            _description,
            _operator
        )
        .fetch_one(&mut **transaction)
        .await?;

        for worker_group in _worker_groups {
            sqlx::query!(
                r#"
                insert into t_ds_environment_worker_group_relation (environment_code,worker_group,operator)
                values ($1,$2,$3)
                "#,
                _code,
                worker_group,
                _operator
            )
            .execute(&mut **transaction)
            .await?;
        }

        Ok(res)
    }

    pub async fn create_and_relation(
        _name: &str,
        _config: &str,
        _description: Option<String>,
        _operator: i32,
        _worker_groups: Vec<String>,
        pool: &PgPool,
    ) -> Result<Self> {
        let mut transtion = pool.begin().await?;
        match Self::insert(_name, _config, _description, _operator, _worker_groups, &mut transtion).await {
            Ok(v) => {
                transtion.commit().await?;
                Ok(v)
            }
            Err(e) => {
                transtion.rollback().await?;
                Err(e)
            }
        }
    }

    async fn delete_by_code(_code: i64, transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            delete from t_ds_environment_worker_group_relation where environment_code = $1
            "#,
            _code
        )
        .execute(&mut **transaction)
        .await?;
        sqlx::query!(
            r#"
            delete from t_ds_environment where code = $1
            "#,
            _code
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
    pub async fn delete_by_code_and_relation(_code: i64, pool: &PgPool) -> Result<()> {
        let mut transtion = pool.begin().await?;

        match Self::delete_by_code(_code, &mut transtion).await {
            Ok(v) => {
                transtion.commit().await?;
                Ok(v)
            }
            Err(e) => {
                transtion.rollback().await?;
                Err(e)
            }
        }
    }
    pub async fn _all(pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_environment
            "#
        )
        .fetch_all(pool)
        .await?)
    }
}
