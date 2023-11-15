use std::{collections::HashMap, str::FromStr};

/**
 * user type
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
impl FromStr for UserType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN_USER" => Ok(UserType::ADMIN_USER),
            "GENERAL_USER" => Ok(UserType::GENERAL_USER),
            _ => Err(()),
        }
    }
}
impl From<String> for UserType {
    fn from(value: String) -> Self {
        let a: UserType = value.parse().unwrap();
        a
    }
}

#[test]
fn test() {
    let a = UserType::ADMIN_USER;
    let b = a.get_code();
    println!("{}", b);
    let c = a.get_descp();
    println!("{}", c);
    let d = a.get_message();
    println!("{:?}", d);
    let e = UserType::of(&0);
    println!("{:?}", e);
    let f = UserType::from_str("ADMIN_USER");
    println!("{:?}", f);
    let g = UserType::from("ADMIN_USER".to_string());
    println!("{:?}", g);
}
