use crate::cmd::message::Message;
#[cfg(test)]
pub mod test;
pub trait RequestProcessor {
    fn process(&self, msg: &Message);
}

pub trait MasterRpcProcessor: RequestProcessor {
    // fn process(&self, msg: &Message);
}

pub struct BaseLogProcessor;
pub struct WorkflowMetricsCleanUpProcessor;

impl RequestProcessor for BaseLogProcessor {
    fn process(&self, msg: &Message) {
        println!("{:?}", msg);
    }
}

impl MasterRpcProcessor for BaseLogProcessor {
    // fn process(&self, msg: &Message) {
    //     println!("BaseLogProcessor process");
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base_log_processor() {
        let base_log_processor = BaseLogProcessor;
        base_log_processor.process(&Message {
            id: 1,
            data: "test".as_bytes().to_vec(),
            message_type: crate::cmd::message_type::MessageType::CACHE_EXPIRE,
            opaque: 1,
        });
    }
}
