use anyhow::Result;
use chrono::NaiveDateTime;
pub struct Model {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub version: i32,
    pub description: Option<String>,
    pub project_code: Option<i64>,
    pub release_state: Option<i32>,
    pub user_id: Option<i32>,
    pub global_params: Option<String>,
    pub locations: Option<String>,
    pub warning_group_id: Option<i32>,
    pub flag: Option<i32>,
    pub timeout: Option<i32>,
    pub execution_type: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
impl Model {
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn _create(
        code: i64,
        name: String,
        version: i32,
        description: String,
        project_code: i64,
        release_state: i32,
        user_id: i32,
        global_params: String,
        locations: String,
        warning_group_id: i32,
        flag: i32,
        timeout: i32,
        execution_type: i32,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "insert into t_ds_process_definition \
             (code,name,version,description,project_code,release_state,user_id,global_params,locations,\
             warning_group_id,flag,timeout,execution_type ) values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) \
             returning *",
            code,
            name,
            version,
            description,
            project_code,
            release_state,
            user_id,
            global_params,
            locations,
            warning_group_id,
            flag,
            timeout,
            execution_type
        )
        .fetch_one(pool)
        .await?)
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn _update(
        code: i64,
        name: String,
        version: i32,
        description: String,
        project_code: i64,
        release_state: i32,
        user_id: i32,
        global_params: String,
        locations: String,
        warning_group_id: i32,
        flag: i32,
        timeout: i32,
        execution_type: i32,
        pool: &sqlx::PgPool,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
           update t_ds_process_definition set name=$1,version=$2,description=$3,project_code=$4,release_state=$5,user_id=$6,global_params=$7,locations=$8,warning_group_id=$9,flag=$10,timeout=$11,execution_type=$12 where code=$13 returning *
            "#,
            name,
            version,
            description,
            project_code,
            release_state,
            user_id,
            global_params,
            locations,
            warning_group_id,
            flag,
            timeout,
            execution_type,
            code
        )
        .fetch_one(pool)
        .await?)
    }
}
