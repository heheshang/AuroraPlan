use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum SparkVersion {
    /**
     * 0 SPARK1
     * 1 SPARK2
     * 2 SPARKSQL
     */
    SPARK1,
    SPARK2,
    SPARKSQL,
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
/// let vikings = HashMap::from([
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///     (Viking::new("Harald", "Iceland"), 12),
/// ]);

impl SparkVersion {
    pub fn get_message(&self) -> Message {
        match self {
            SparkVersion::SPARK1 => Message::new(0, "SPARK1".to_string()),
            SparkVersion::SPARK2 => Message::new(1, "SPARK2".to_string()),
            SparkVersion::SPARKSQL => Message::new(2, "SPARKSQL".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> SparkVersion {
        let MAP: HashMap<i32, SparkVersion> = HashMap::from([
            (0, SparkVersion::SPARK1),
            (1, SparkVersion::SPARK2),
            (2, SparkVersion::SPARKSQL),
        ]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<SparkVersion> for Message {
    fn from(alert_status: SparkVersion) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for SparkVersion {
    fn from(value: Message) -> Self {
        let a: SparkVersion = value.into();
        a
    }
}
