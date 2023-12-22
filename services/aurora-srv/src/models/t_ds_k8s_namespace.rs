use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

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
