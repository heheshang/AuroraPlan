use std::collections::HashMap;

/**
 * running status for task group queue
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum TaskGroupQueueStatus {
    WAIT_QUEUE,
    ACQUIRE_SUCCESS,
    RELEASE,
}

#[derive(Clone, Debug)]
pub struct Message {
    code: i32,
    descp: String,
}

impl Message {
    fn new(
        code: i32,
        descp: String,
    ) -> Message {
        Message { code, descp }
    }
}

impl TaskGroupQueueStatus {
    pub fn get_message(&self) -> Message {
        match self {
            TaskGroupQueueStatus::WAIT_QUEUE => Message::new(-1, "wait queue".to_string()),
            TaskGroupQueueStatus::ACQUIRE_SUCCESS => Message::new(1, "acquire success".to_string()),
            TaskGroupQueueStatus::RELEASE => Message::new(2, "release".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> TaskGroupQueueStatus {
        let MAP: HashMap<i32, TaskGroupQueueStatus> = HashMap::from([
            (-1, TaskGroupQueueStatus::WAIT_QUEUE),
            (1, TaskGroupQueueStatus::ACQUIRE_SUCCESS),
            (2, TaskGroupQueueStatus::RELEASE),
        ]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<TaskGroupQueueStatus> for Message {
    fn from(alert_status: TaskGroupQueueStatus) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for TaskGroupQueueStatus {
    fn from(value: Message) -> Self {
        let a: TaskGroupQueueStatus = value.into();
        a
    }
}
