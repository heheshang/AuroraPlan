//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_task_definition_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub version: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub project_code: Option<i64>,
    pub user_id: Option<i32>,
    pub task_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub task_params: Option<String>,
    pub flag: Option<i32>,
    pub task_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub environment_code: Option<i64>,
    pub fail_retry_times: Option<i32>,
    pub fail_retry_interval: Option<i32>,
    pub timeout_flag: Option<i32>,
    pub timeout_notify_strategy: Option<i32>,
    pub timeout: Option<i32>,
    pub delay_time: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub resource_ids: Option<String>,
    pub operator: Option<i32>,
    pub task_group_id: Option<i32>,
    pub task_group_priority: Option<i32>,
    pub operate_time: Option<DateTime>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
