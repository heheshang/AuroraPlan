use std::collections::HashMap;

/**
 * UDF type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum UdfType {
    /**
     * 0 hive; 1 spark
     */
    HIVE,
    SPARK,
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

impl UdfType {
    pub fn get_message(&self) -> Message {
        match self {
            UdfType::HIVE => Message::new(0, "hive".to_string()),
            UdfType::SPARK => Message::new(1, "spark".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> UdfType {
        let MAP: HashMap<i32, UdfType> = HashMap::from([(0, UdfType::HIVE), (1, UdfType::SPARK)]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid UdfType operation type code {}", status)
    }
}

impl From<UdfType> for Message {
    fn from(alert_status: UdfType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for UdfType {
    fn from(value: Message) -> Self {
        let a: UdfType = value.into();
        a
    }
}
