#[allow(non_camel_case_types)]
/**
 * complement data run mode
 */
#[derive(Clone, Debug)]
pub enum RunMode {
    /**
     * 0 serial run
     * 1 parallel run
     * */
    RUN_MODE_SERIAL,
    RUN_MODE_PARALLEL,
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

impl RunMode {
    pub fn get_message(&self) -> Message {
        match self {
            RunMode::RUN_MODE_SERIAL => Message::new(0, "serial run".to_string()),
            RunMode::RUN_MODE_PARALLEL => Message::new(1, "parallel run".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<RunMode> for Message {
    fn from(alert_status: RunMode) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for RunMode {
    fn from(value: Message) -> Self {
        let a: RunMode = value.into();
        a
    }
}
