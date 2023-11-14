//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_k8s_namespace")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: i64,
    pub limits_memory: Option<i32>,
    pub namespace: Option<String>,
    pub user_id: Option<i32>,
    pub pod_replicas: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((13, 4)))", nullable)]
    pub pod_request_cpu: Option<Decimal>,
    pub pod_request_memory: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((13, 4)))", nullable)]
    pub limits_cpu: Option<Decimal>,
    pub cluster_code: i64,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
