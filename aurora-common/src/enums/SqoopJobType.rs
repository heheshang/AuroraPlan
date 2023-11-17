use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum SqoopJobType {
    CUSTOM,
    TEMPLATE,
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

impl SqoopJobType {
    pub fn get_message(&self) -> Message {
        match self {
            SqoopJobType::CUSTOM => Message::new(0, "CUSTOM".to_string()),
            SqoopJobType::TEMPLATE => Message::new(1, "TEMPLATE".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> SqoopJobType {
        let MAP: HashMap<i32, SqoopJobType> = HashMap::from([(0, SqoopJobType::CUSTOM), (1, SqoopJobType::TEMPLATE)]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<SqoopJobType> for Message {
    fn from(alert_status: SqoopJobType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for SqoopJobType {
    fn from(value: Message) -> Self {
        let a: SqoopJobType = value.into();
        a
    }
}
