use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_job_details)]
#[diesel(primary_key(sched_name, job_name, job_group))]
pub struct Model {
    pub sched_name: String,
    pub job_name: String,
    pub job_group: String,
    pub description: Option<String>,
    pub job_class_name: String,
    pub is_durable: bool,
    pub is_nonconcurrent: bool,
    pub is_update_data: bool,
    pub requests_recovery: bool,
    pub job_data: Option<Vec<u8>>,
}
