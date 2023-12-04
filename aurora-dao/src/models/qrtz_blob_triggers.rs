use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::qrtz_blob_triggers)]
pub struct BlobTriggers {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub blob_data: Option<Vec<u8>>,
}
