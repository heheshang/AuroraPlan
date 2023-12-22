pub struct BlobTriggers {
    pub sched_name: String,
    pub trigger_name: String,
    pub trigger_group: String,
    pub blob_data: Option<Vec<u8>>,
}
