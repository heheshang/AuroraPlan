use std::collections::HashMap;

/**
 * Audit Operation type
 */
#[derive(Clone, Debug)]
pub enum AuditOperationType {
    CREATE,
    READ,
    UPDATE,
    DELETE,
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
///

impl AuditOperationType {
    pub fn get_message(&self) -> Message {
        match self {
            AuditOperationType::CREATE => Message::new(0, "CREATE".to_string()),
            AuditOperationType::READ => Message::new(1, "READ".to_string()),
            AuditOperationType::UPDATE => Message::new(2, "UPDATE".to_string()),
            AuditOperationType::DELETE => Message::new(3, "DELETE".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> AuditOperationType {
        let audit_operation_map: HashMap<i32, AuditOperationType> = HashMap::from([
            (0, AuditOperationType::CREATE),
            (1, AuditOperationType::READ),
            (2, AuditOperationType::UPDATE),
            (3, AuditOperationType::DELETE),
        ]);

        if audit_operation_map.contains_key(status) {
            return audit_operation_map.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<AuditOperationType> for Message {
    fn from(alert_status: AuditOperationType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for AuditOperationType {
    fn from(value: Message) -> Self {
        let a: AuditOperationType = value.into();
        a
    }
}
