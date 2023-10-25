#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum BlockingOpportunity {
    BLOCKING_ON_SUCCESS,
    BLOCKING_ON_FAILED,
}

pub struct Message {
    descp: String,
}

impl Message {
    fn new(descp: String) -> Message {
        Message { descp }
    }
}

impl BlockingOpportunity {
    pub fn get_message(&self) -> Message {
        match self {
            BlockingOpportunity::BLOCKING_ON_SUCCESS => {
                Message::new("BlockingOnSuccess".to_string())
            }
            BlockingOpportunity::BLOCKING_ON_FAILED => Message::new("BlockingOnFailed".to_string()),
        }
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }
}

impl From<BlockingOpportunity> for Message {
    fn from(alert_status: BlockingOpportunity) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for BlockingOpportunity {
    fn from(value: Message) -> Self {
        let a: BlockingOpportunity = value.into();
        a
    }
}
