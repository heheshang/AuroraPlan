use std::collections::HashMap;

/**
 * condition type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum ConditionType {
    /**
     * 0 none
     * 1 judge
     * 2 delay
     */
    NONE,
    JUDGE,
    DELAY,
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
/// let vikings = HashMap::from([
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///     (Viking::new("Harald", "Iceland"), 12),
/// ]);

impl ConditionType {
    pub fn get_message(&self) -> Message {
        match self {
            ConditionType::NONE => Message::new(0, "none".to_string()),
            ConditionType::JUDGE => Message::new(1, "judge".to_string()),
            ConditionType::DELAY => Message::new(2, "delay".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> ConditionType {
        let map: HashMap<i32, ConditionType> = HashMap::from([
            (0, ConditionType::NONE),
            (1, ConditionType::JUDGE),
            (2, ConditionType::DELAY),
        ]);
        if map.contains_key(status) {
            return map.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<ConditionType> for Message {
    fn from(alert_status: ConditionType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for ConditionType {
    fn from(value: Message) -> Self {
        let a: ConditionType = value.into();
        a
    }
}
