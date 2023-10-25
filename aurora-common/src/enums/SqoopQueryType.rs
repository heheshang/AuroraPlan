use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]

pub enum SqoopQueryType {
    FORM,
    SQL,
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
/// let vikings = HashMap::from([
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///
///     (Viking::new("Harald", "Iceland"), 12),
/// ]);

impl SqoopQueryType {
    pub fn get_message(&self) -> Message {
        match self {
            SqoopQueryType::FORM => Message::new(0, "FORM".to_string()),
            SqoopQueryType::SQL => Message::new(1, "SQL".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> SqoopQueryType {
        let MAP: HashMap<i32, SqoopQueryType> =
            HashMap::from([(0, SqoopQueryType::FORM), (1, SqoopQueryType::SQL)]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<SqoopQueryType> for Message {
    fn from(alert_status: SqoopQueryType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for SqoopQueryType {
    fn from(value: Message) -> Self {
        let a: SqoopQueryType = value.into();
        a
    }
}
