/**
 * have_script
 * have_file
 * can_retry
 * have_arr_variables
 * have_map_variables
 * have_alert
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Flag {
    /**
     * 0 no
     * 1 yes
     */
    NO,
    YES,
}

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

impl Flag {
    pub fn get_message(&self) -> Message {
        match self {
            Flag::NO => Message::new(0, "no".to_string()),
            Flag::YES => Message::new(1, "yes".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<Flag> for Message {
    fn from(alert_status: Flag) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for Flag {
    fn from(value: Message) -> Self {
        let a: Flag = value.into();
        a
    }
}
