//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_task_instance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub task_type: Option<String>,
    pub task_code: i64,
    pub task_definition_version: Option<i32>,
    pub process_instance_id: Option<i32>,
    pub state: Option<i32>,
    pub submit_time: Option<DateTime>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub host: Option<String>,
    pub execute_path: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub log_path: Option<String>,
    pub alert_flag: Option<i32>,
    pub retry_times: Option<i32>,
    pub pid: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub app_link: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub task_params: Option<String>,
    pub flag: Option<i32>,
    pub retry_interval: Option<i32>,
    pub max_retry_times: Option<i32>,
    pub task_instance_priority: Option<i32>,
    pub worker_group: Option<String>,
    pub environment_code: Option<i64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub environment_config: Option<String>,
    pub executor_id: Option<i32>,
    pub first_submit_time: Option<DateTime>,
    pub delay_time: Option<i32>,
    pub task_group_id: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub var_pool: Option<String>,
    pub dry_run: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::t_ds_process_instance::Entity",
        from = "Column::ProcessInstanceId",
        to = "super::t_ds_process_instance::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TDsProcessInstance,
}

impl Related<super::t_ds_process_instance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TDsProcessInstance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
