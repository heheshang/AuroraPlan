#[allow(non_camel_case_types)]

/**
* describe the reason why alert generates

*/
#[derive(Clone, Debug)]
pub enum AlertType {
    /**
     * 0 process instance failure, 1 process instance success, 2 process instance blocked, 3 process instance timeout, 4 fault tolerance warning,
     * 5 task failure, 6 task success, 7 task timeout
     */
    PROCESS_INSTANCE_FAILURE,
    PROCESS_INSTANCE_SUCCESS,
    PROCESS_INSTANCE_BLOCKED,
    PROCESS_INSTANCE_TIMEOUT,
    FAULT_TOLERANCE_WARNING,
    TASK_FAILURE,
    TASK_SUCCESS,
    TASK_TIMEOUT,
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

impl AlertType {
    pub fn get_message(&self) -> Message {
        match self {
            AlertType::PROCESS_INSTANCE_FAILURE => Message::new(0, "process instance failure".to_string()),
            AlertType::PROCESS_INSTANCE_SUCCESS => Message::new(1, "process instance success".to_string()),
            AlertType::PROCESS_INSTANCE_BLOCKED => Message::new(2, "process instance blocked".to_string()),
            AlertType::PROCESS_INSTANCE_TIMEOUT => Message::new(3, "process instance timeout".to_string()),
            AlertType::FAULT_TOLERANCE_WARNING => Message::new(4, "fault tolerance warning".to_string()),
            AlertType::TASK_FAILURE => Message::new(5, "task failure".to_string()),
            AlertType::TASK_SUCCESS => Message::new(6, "task success".to_string()),
            AlertType::TASK_TIMEOUT => Message::new(7, "task timeout".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<AlertType> for Message {
    fn from(alert_status: AlertType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for AlertType {
    fn from(value: Message) -> Self {
        let a: AlertType = value.into();
        a
    }
}
