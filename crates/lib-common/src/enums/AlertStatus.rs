#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum AlertStatus {
    /**
     * 0 waiting executed; 1 execute successfully，2 execute failed
     */
    // WAIT_EXECUTION(0, "waiting executed"),
    // EXECUTION_SUCCESS(1, "execute successfully"),
    // EXECUTION_FAILURE(2, "execute failed"),
    // EXECUTION_PARTIAL_SUCCESS(3, "execute partial successfully");
    WAIT_EXECUTION,
    EXECUTION_SUCCESS,
    EXECUTION_FAILURE,
    EXECUTION_PARTIAL_SUCCESS,
}
pub struct Message {
    code: i32,
    descp: String,
}

impl Message {
    fn new(code: i32, descp: String) -> Message {
        Message { code, descp }
    }
}

impl AlertStatus {
    pub fn get_message(&self) -> Message {
        match self {
            AlertStatus::WAIT_EXECUTION => Message::new(0, "waiting executed".to_string()),
            AlertStatus::EXECUTION_SUCCESS => Message::new(1, "execute successfully".to_string()),
            AlertStatus::EXECUTION_FAILURE => Message::new(2, "execute failed".to_string()),
            AlertStatus::EXECUTION_PARTIAL_SUCCESS => Message::new(3, "execute partial successfully".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<AlertStatus> for Message {
    fn from(alert_status: AlertStatus) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for AlertStatus {
    fn from(value: Message) -> Self {
        let a: AlertStatus = value.into();
        a
    }
}
