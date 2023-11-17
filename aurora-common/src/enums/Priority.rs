/**
 *  define process and task priority
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Priority {
    /**
     * 0 highest priority
     * 1 higher priority
     * 2 medium priority
     * 3 lower priority
     * 4 lowest priority
     */
    HIGHEST,
    HIGH,
    MEDIUM,
    LOW,
    LOWEST,
}

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

impl Priority {
    pub fn get_message(&self) -> Message {
        match self {
            Priority::HIGHEST => Message::new(0, "highest".to_string()),
            Priority::HIGH => Message::new(1, "high".to_string()),
            Priority::MEDIUM => Message::new(2, "medium".to_string()),
            Priority::LOW => Message::new(3, "low".to_string()),
            Priority::LOWEST => Message::new(4, "lowest".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<Priority> for Message {
    fn from(alert_status: Priority) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for Priority {
    fn from(value: Message) -> Self {
        let a: Priority = value.into();
        a
    }
}
