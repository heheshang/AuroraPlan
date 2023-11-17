use std::collections::HashMap;

/**
 * types for whether to send warning when process ends;
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum WarningType {
    /**
     * 0 do not send warning;
     * 1 send if process success;
     * 2 send if process failed;
     * 3 send if process ends, whatever the result;
     */
    NONE,
    SUCCESS,
    FAILURE,
    ALL,
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

impl WarningType {
    pub fn get_message(&self) -> Message {
        match self {
            WarningType::NONE => Message::new(0, "none".to_string()),
            WarningType::SUCCESS => Message::new(1, "success".to_string()),
            WarningType::FAILURE => Message::new(2, "failure".to_string()),
            WarningType::ALL => Message::new(3, "all".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> WarningType {
        let MAP: HashMap<i32, WarningType> = HashMap::from([
            (0, WarningType::NONE),
            (1, WarningType::SUCCESS),
            (2, WarningType::FAILURE),
            (3, WarningType::ALL),
        ]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<WarningType> for Message {
    fn from(alert_status: WarningType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for WarningType {
    fn from(value: Message) -> Self {
        let a: WarningType = value.into();
        a
    }
}
