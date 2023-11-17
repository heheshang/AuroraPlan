/**
 * failure policy when some task node failed.
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum FailureStrategy {
    /**
     * 0 ending process when some tasks failed.
     * 1 continue running when some tasks failed.
     **/
    END,
    CONTINUE,
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

impl FailureStrategy {
    pub fn get_message(&self) -> Message {
        match self {
            FailureStrategy::END => Message::new(0, "end".to_string()),
            FailureStrategy::CONTINUE => Message::new(1, "continue".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<FailureStrategy> for Message {
    fn from(alert_status: FailureStrategy) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for FailureStrategy {
    fn from(value: Message) -> Self {
        let a: FailureStrategy = value.into();
        a
    }
}
