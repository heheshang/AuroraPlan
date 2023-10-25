use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]

pub enum ProcessExecutionTypeEnum {
    PARALLEL,
    SERIAL_WAIT,
    SERIAL_DISCARD,
    SERIAL_PRIORITY,
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

impl ProcessExecutionTypeEnum {
    pub fn get_message(&self) -> Message {
        match self {
            ProcessExecutionTypeEnum::PARALLEL => Message::new(0, "parallel".to_string()),
            ProcessExecutionTypeEnum::SERIAL_WAIT => Message::new(1, "serial wait".to_string()),
            ProcessExecutionTypeEnum::SERIAL_DISCARD => {
                Message::new(2, "serial discard".to_string())
            }
            ProcessExecutionTypeEnum::SERIAL_PRIORITY => {
                Message::new(3, "serial priority".to_string())
            }
        }
    }

    pub fn typeIsSerial(&self) -> bool {
        matches!(self, ProcessExecutionTypeEnum::PARALLEL)
    }

    pub fn typeIsSerialWait(&self) -> bool {
        matches!(self, ProcessExecutionTypeEnum::SERIAL_WAIT)
    }

    pub fn typeIsSerialDiscard(&self) -> bool {
        matches!(self, ProcessExecutionTypeEnum::SERIAL_DISCARD)
    }

    pub fn typeIsSerialPriority(&self) -> bool {
        matches!(self, ProcessExecutionTypeEnum::SERIAL_PRIORITY)
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> ProcessExecutionTypeEnum {
        let EXECUTION_STATUS_MAP: HashMap<i32, ProcessExecutionTypeEnum> = HashMap::from([
            (0, ProcessExecutionTypeEnum::PARALLEL),
            (1, ProcessExecutionTypeEnum::SERIAL_WAIT),
            (2, ProcessExecutionTypeEnum::SERIAL_DISCARD),
            (3, ProcessExecutionTypeEnum::SERIAL_PRIORITY),
        ]);
        if EXECUTION_STATUS_MAP.contains_key(status) {
            return EXECUTION_STATUS_MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<ProcessExecutionTypeEnum> for Message {
    fn from(alert_status: ProcessExecutionTypeEnum) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for ProcessExecutionTypeEnum {
    fn from(value: Message) -> Self {
        let a: ProcessExecutionTypeEnum = value.into();
        a
    }
}
