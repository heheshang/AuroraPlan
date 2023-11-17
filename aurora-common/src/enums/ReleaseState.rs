/**
 * process define release state
 */
pub enum ReleaseState {
    /**
     * 0 offline
     * 1 online
     */
    OFFLINE,
    ONLINE,
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

impl ReleaseState {
    pub fn get_message(&self) -> Message {
        match self {
            ReleaseState::OFFLINE => Message::new(0, "offline".to_string()),
            ReleaseState::ONLINE => Message::new(1, "online".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<ReleaseState> for Message {
    fn from(alert_status: ReleaseState) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for ReleaseState {
    fn from(value: Message) -> Self {
        let a: ReleaseState = value.into();
        a
    }
}
