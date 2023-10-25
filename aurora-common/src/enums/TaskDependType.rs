use std::collections::HashMap;

/**
 * task node depend type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum TaskDependType {
    /**
     * 0 run current tasks only
     * 1 run current tasks and previous tasks
     * 2 run current tasks and the other tasks that depend on current tasks;
     */
    TASK_ONLY,
    TASK_PRE,
    TASK_POST,
}

#[derive(Clone, Debug)]
pub struct Message {
    code: i32,
    descp: String,
}

impl Message {
    fn new(code: i32, descp: String) -> Message {
        Message { code, descp }
    }
}
/// let vikings = HashMap::from([
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///
///     (Viking::new("Harald", "Iceland"), 12),
/// ]);

impl TaskDependType {
    pub fn get_message(&self) -> Message {
        match self {
            TaskDependType::TASK_ONLY => Message::new(0, "task only".to_string()),
            TaskDependType::TASK_PRE => Message::new(1, "task pre".to_string()),
            TaskDependType::TASK_POST => Message::new(2, "task post".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> TaskDependType {
        let MAP: HashMap<i32, TaskDependType> = HashMap::from([
            (0, TaskDependType::TASK_ONLY),
            (1, TaskDependType::TASK_PRE),
            (2, TaskDependType::TASK_POST),
        ]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<TaskDependType> for Message {
    fn from(alert_status: TaskDependType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for TaskDependType {
    fn from(value: Message) -> Self {
        let a: TaskDependType = value.into();
        a
    }
}
