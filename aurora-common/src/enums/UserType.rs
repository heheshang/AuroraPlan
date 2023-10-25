use std::collections::HashMap;

/**
 * user type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum UserType {
    /**
     * 0 admin user; 1 general user
     */
    ADMIN_USER,
    GENERAL_USER,
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

impl UserType {
    pub fn get_message(&self) -> Message {
        match self {
            UserType::ADMIN_USER => Message::new(0, "admin user".to_string()),
            UserType::GENERAL_USER => Message::new(1, "general user".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> UserType {
        let MAP: HashMap<i32, UserType> =
            HashMap::from([(0, UserType::ADMIN_USER), (1, UserType::GENERAL_USER)]);
        if MAP.contains_key(status) {
            return MAP.get(status).unwrap().clone();
        }
        panic!("invalid UserType operation type code {}", status)
    }
}

impl From<UserType> for Message {
    fn from(alert_status: UserType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for UserType {
    fn from(value: Message) -> Self {
        let a: UserType = value.into();
        a
    }
}
