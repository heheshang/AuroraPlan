use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum CommandType {
    /**
     * command types
     * 0 start a new process
     * 1 start a new process from current nodes
     * 2 recover tolerance fault process
     * 3 recover suspended process
     * 4 start process from failure task nodes
     * 5 complement data
     * 6 start a new process from scheduler
     * 7 repeat running a process
     * 8 pause a process
     * 9 stop a process
     * 10 recover waiting thread
     */
    START_PROCESS,
    START_CURRENT_TASK_PROCESS,
    RECOVER_TOLERANCE_FAULT_PROCESS,
    RECOVER_SUSPENDED_PROCESS,
    START_FAILURE_TASK_PROCESS,
    COMPLEMENT_DATA,
    SCHEDULER,
    REPEAT_RUNNING,
    PAUSE,
    STOP,
    RECOVER_WAITING_THREAD,
    RECOVER_SERIAL_WAIT,
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

impl CommandType {
    pub fn get_message(&self) -> Message {
        match self {
            CommandType::START_PROCESS => Message::new(0, "start a new process".to_string()),
            CommandType::START_CURRENT_TASK_PROCESS => {
                Message::new(1, "start a new process from current nodes".to_string())
            }
            CommandType::RECOVER_TOLERANCE_FAULT_PROCESS => {
                Message::new(2, "recover tolerance fault process".to_string())
            }
            CommandType::RECOVER_SUSPENDED_PROCESS => {
                Message::new(3, "recover suspended process".to_string())
            }
            CommandType::START_FAILURE_TASK_PROCESS => {
                Message::new(4, "start process from failure task nodes".to_string())
            }
            CommandType::COMPLEMENT_DATA => Message::new(5, "complement data".to_string()),
            CommandType::SCHEDULER => {
                Message::new(6, "start a new process from scheduler".to_string())
            }
            CommandType::REPEAT_RUNNING => Message::new(7, "repeat running a process".to_string()),
            CommandType::PAUSE => Message::new(8, "pause a process".to_string()),
            CommandType::STOP => Message::new(9, "stop a process".to_string()),
            CommandType::RECOVER_WAITING_THREAD => {
                Message::new(10, "recover waiting thread".to_string())
            }
            CommandType::RECOVER_SERIAL_WAIT => Message::new(11, "recover serial wait".to_string()),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.get_message().code
    }

    pub fn get_descp(&self) -> String {
        self.get_message().descp
    }

    pub fn of(status: &i32) -> CommandType {
        let map: HashMap<i32, CommandType> = HashMap::from([
            (0, CommandType::START_PROCESS),
            (1, CommandType::START_CURRENT_TASK_PROCESS),
            (2, CommandType::RECOVER_TOLERANCE_FAULT_PROCESS),
            (3, CommandType::RECOVER_SUSPENDED_PROCESS),
            (4, CommandType::START_FAILURE_TASK_PROCESS),
            (5, CommandType::COMPLEMENT_DATA),
            (6, CommandType::SCHEDULER),
            (7, CommandType::REPEAT_RUNNING),
            (8, CommandType::PAUSE),
            (9, CommandType::STOP),
            (10, CommandType::RECOVER_WAITING_THREAD),
            (11, CommandType::RECOVER_SERIAL_WAIT),
        ]);
        if map.contains_key(status) {
            return map.get(status).unwrap().clone();
        }
        panic!("invalid audit operation type code {}", status)
    }
}

impl From<CommandType> for Message {
    fn from(alert_status: CommandType) -> Self {
        alert_status.get_message()
    }
}
impl From<Message> for CommandType {
    fn from(value: Message) -> Self {
        let a: CommandType = value.into();
        a
    }
}
