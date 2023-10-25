#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum DbConnectType {
    ORACLE_SERVICE_NAME,
    ORACLE_SID,
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

impl DbConnectType {
    pub fn get_message(&self) -> Message {
        match self {
            DbConnectType::ORACLE_SERVICE_NAME => {
                Message::new(0, "Oracle Service Name".to_string())
            }
            DbConnectType::ORACLE_SID => Message::new(1, "Oracle SID".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<DbConnectType> for Message {
    fn from(alert_status: DbConnectType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for DbConnectType {
    fn from(value: Message) -> Self {
        let a: DbConnectType = value.into();
        a
    }
}
