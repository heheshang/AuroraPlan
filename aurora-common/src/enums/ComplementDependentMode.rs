/**
 * task node depend type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum ComplementDependentMode {
    /**
     * 0 off mode
     * 1 run complement data with all dependent process
     */
    OFF_MODE,
    ALL_DEPENDENT,
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

impl ComplementDependentMode {
    pub fn get_message(&self) -> Message {
        match self {
            ComplementDependentMode::OFF_MODE => Message::new(0, "off mode".to_string()),
            ComplementDependentMode::ALL_DEPENDENT => Message::new(1, "all dependent".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<ComplementDependentMode> for Message {
    fn from(alert_status: ComplementDependentMode) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for ComplementDependentMode {
    fn from(value: Message) -> Self {
        let a: ComplementDependentMode = value.into();
        a
    }
}
