#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum CacheType {
    TENANT,
    USER,
    QUEUE,
    PROCESS_DEFINITION,
    PROCESS_TASK_RELATION,
    TASK_DEFINITION,
    WORKER_GROUP,
    SCHEDULE,
}

pub struct Message {
    cache_name: String,
}

impl Message {
    fn new(cache_name: String) -> Message {
        Message { cache_name }
    }
}

impl CacheType {
    pub fn get_message(&self) -> Message {
        match self {
            CacheType::TENANT => Message::new("tenant".to_string()),
            CacheType::USER => Message::new("user".to_string()),
            CacheType::QUEUE => Message::new("queue".to_string()),
            CacheType::PROCESS_DEFINITION => Message::new("processDefinition".to_string()),
            CacheType::PROCESS_TASK_RELATION => Message::new("processTaskRelation".to_string()),
            CacheType::TASK_DEFINITION => Message::new("taskDefinition".to_string()),
            CacheType::WORKER_GROUP => Message::new("workerGroup".to_string()),
            CacheType::SCHEDULE => Message::new("schedule".to_string()),
        }
    }

    pub fn get_cache_name(&self) -> String {
        self.get_message().cache_name
    }
}

impl From<CacheType> for Message {
    fn from(alert_status: CacheType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for CacheType {
    fn from(value: Message) -> Self {
        let a: CacheType = value.into();
        a
    }
}
