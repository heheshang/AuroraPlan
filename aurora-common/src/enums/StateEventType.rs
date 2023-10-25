use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]

pub enum StateEventType {
    PROCESS_STATE_CHANGE,
    TASK_STATE_CHANGE,
    PROCESS_TIMEOUT,
    TASK_TIMEOUT,
    WAIT_TASK_GROUP,
    TASK_RETRY,
    PROCESS_BLOCKED,
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

impl StateEventType {
    pub fn get_message(&self) -> Message {
        match self {
            StateEventType::PROCESS_STATE_CHANGE => {
                Message::new(0, "PROCESS_STATE_CHANGE".to_string())
            }
            StateEventType::TASK_STATE_CHANGE => Message::new(1, "TASK_STATE_CHANGE".to_string()),
            StateEventType::PROCESS_TIMEOUT => Message::new(2, "PROCESS_TIMEOUT".to_string()),
            StateEventType::TASK_TIMEOUT => Message::new(3, "TASK_TIMEOUT".to_string()),
            StateEventType::WAIT_TASK_GROUP => Message::new(4, "WAIT_TASK_GROUP".to_string()),
            StateEventType::TASK_RETRY => Message::new(5, "TASK_RETRY".to_string()),
            StateEventType::PROCESS_BLOCKED => Message::new(6, "PROCESS_BLOCKED".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> StateEventType {
        let MAP: HashMap<i32, StateEventType> = HashMap::from([
            (0, StateEventType::PROCESS_STATE_CHANGE),
            (1, StateEventType::TASK_STATE_CHANGE),
            (2, StateEventType::PROCESS_TIMEOUT),
            (3, StateEventType::TASK_TIMEOUT),
            (4, StateEventType::WAIT_TASK_GROUP),
            (5, StateEventType::TASK_RETRY),
            (6, StateEventType::PROCESS_BLOCKED),
        ]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<StateEventType> for Message {
    fn from(alert_status: StateEventType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for StateEventType {
    fn from(value: Message) -> Self {
        let a: StateEventType = value.into();
        a
    }
}
