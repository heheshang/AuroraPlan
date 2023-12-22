use std::collections::HashMap;

/**
 * timeout flag
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum TimeoutFlag {
    /**
     * 0 close
     * 1 open
     */
    CLOSE,
    OPEN,
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

impl TimeoutFlag {
    pub fn get_message(&self) -> Message {
        match self {
            TimeoutFlag::CLOSE => Message::new(0, "close".to_string()),
            TimeoutFlag::OPEN => Message::new(1, "open".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> TimeoutFlag {
        let MAP: HashMap<i32, TimeoutFlag> = HashMap::from([(0, TimeoutFlag::CLOSE), (1, TimeoutFlag::OPEN)]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<TimeoutFlag> for Message {
    fn from(alert_status: TimeoutFlag) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for TimeoutFlag {
    fn from(value: Message) -> Self {
        let a: TimeoutFlag = value.into();
        a
    }
}
