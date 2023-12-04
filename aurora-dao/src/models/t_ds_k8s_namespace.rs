use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::t_ds_k8s_namespace)]
pub struct Model {
    pub id: i32,
    pub code: i64,
    pub limits_memory: Option<i32>,
    pub namespace: Option<String>,
    pub user_id: Option<i32>,
    pub pod_replicas: Option<i32>,
    pub pod_request_cpu: Option<BigDecimal>,
    pub pod_request_memory: Option<i32>,
    pub limits_cpu: Option<BigDecimal>,
    pub cluster_code: i64,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}
