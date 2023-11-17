use std::collections::HashMap;

/**
 * Audit Module type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum AuditResourceType {
    // TODO: add other audit resource enums
    USER_MODULE,
    PROJECT_MODULE,
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

impl AuditResourceType {
    pub fn get_message(&self) -> Message {
        match self {
            AuditResourceType::USER_MODULE => Message::new(0, "USER_MODULE".to_string()),
            AuditResourceType::PROJECT_MODULE => Message::new(1, "PROJECT_MODULE".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> AuditResourceType {
        let map: HashMap<i32, AuditResourceType> = HashMap::from([
            (0, AuditResourceType::USER_MODULE),
            (1, AuditResourceType::PROJECT_MODULE),
        ]);

        if map.contains_key(status) {
            return map.get(status).unwrap().clone();
        }
        panic!("invalid AuditResourceType operation type code {}", status)
    }
}

impl From<AuditResourceType> for Message {
    fn from(alert_status: AuditResourceType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for AuditResourceType {
    fn from(value: Message) -> Self {
        let a: AuditResourceType = value.into();
        a
    }
}
