use anyhow::Result;
use chrono::NaiveDateTime;
pub struct Model {
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    // #[diesel(unique)]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
impl Model {
    pub(crate) async fn _create(
        alert_instance_ids: String,
        create_user_id: i32,
        group_name: String,
        description: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into t_ds_alertgroup (alert_instance_ids,create_user_id,group_name,description ) values \
             ($1,$2,$3,$4) returning *",
            alert_instance_ids,
            create_user_id,
            group_name,
            description
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _update(
        alert_instance_ids: String,
        create_user_id: i32,
        group_name: String,
        description: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
           update t_ds_alertgroup set alert_instance_ids=$1,create_user_id=$2,group_name=$3,description=$4 where group_name=$3 returning *
            "#,
            alert_instance_ids,
            create_user_id,
            group_name,
            description
        )
        .fetch_one(pool)
        .await?)
    }
    pub(crate) async fn _find_by_name(group_name: String, pool: &sqlx::PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            select * from t_ds_alertgroup where group_name=$1 
            "#,
            group_name
        )
        .fetch_one(pool)
        .await?)
    }
}
