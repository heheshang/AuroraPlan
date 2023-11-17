use std::collections::HashMap;

/**
 * PluginType
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum PluginType {
    ALERT,
    REGISTER,
    TASK,
}

pub struct Message {
    code: i32,
    descp: String,
    hasUi: bool,
}

impl Message {
    fn new(code: i32, descp: String, hasUi: bool) -> Message {
        Message { code, descp, hasUi }
    }
}

impl PluginType {
    pub fn get_message(&self) -> Message {
        match self {
            PluginType::ALERT => Message::new(0, "alert".to_string(), true),
            PluginType::REGISTER => Message::new(1, "register".to_string(), false),
            PluginType::TASK => Message::new(2, "task".to_string(), true),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn hasUi(&self) -> bool {
        self.get_message().hasUi
    }
}

pub fn of(status: &i32) -> PluginType {
    let PLUGIN_TYPE_MAP: HashMap<i32, PluginType> =
        HashMap::from([(0, PluginType::ALERT), (1, PluginType::REGISTER), (2, PluginType::TASK)]);
    if PLUGIN_TYPE_MAP.contains_key(status) {
        return PLUGIN_TYPE_MAP.get(status).unwrap().clone();
    }
    panic!("invalid audit operation type code {}", status)
}
impl From<PluginType> for Message {
    fn from(alert_status: PluginType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for PluginType {
    fn from(value: Message) -> Self {
        let a: PluginType = value.into();
        a
    }
}
