use educe::Educe;
#[allow(non_camel_case_types)]
#[derive(Educe)]
#[educe(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default)]
pub enum MessageType {
    #[default]
    RESPONSE,

    GET_APP_ID_REQUEST,

    REMOVE_TAK_LOG_REQUEST,

    ROLL_VIEW_LOG_REQUEST,

    VIEW_WHOLE_LOG_REQUEST,

    GET_LOG_BYTES_REQUEST,

    /**
     * task execute start, from api to master
     */
    TASK_EXECUTE_START,

    TASK_DISPATCH_REQUEST,
    TASK_DISPATCH_RESPONSE,

    /**
     * task execute running, from worker to master
     */
    TASK_EXECUTE_RUNNING_MESSAGE,

    /**
     * task execute running ack, from master to worker
     */
    TASK_EXECUTE_RUNNING_MESSAGE_ACK,

    /**
     * task execute response, from worker to master
     */
    TASK_EXECUTE_RESULT_MESSAGE,

    /**
     * task execute response ack, from master to worker
     */
    TASK_EXECUTE_RESULT_MESSAGE_ACK,

    TASK_KILL_REQUEST,

    /**
     * task savepoint, for stream task
     */
    TASK_SAVEPOINT_REQUEST,

    HEART_BEAT,

    PING,

    PONG,

    ALERT_SEND_REQUEST,

    WORKFLOW_HOST_CHANGE_REQUEST,

    /**
     * state event request
     */
    STATE_EVENT_REQUEST,
    /**
     * cache expire
     */
    CACHE_EXPIRE,
    /**
     * task state event request
     */
    TASK_FORCE_STATE_EVENT_REQUEST,
    /**
     * task state event request
     */
    TASK_WAKEUP_EVENT_REQUEST,

    /**
     * workflow executing data request, from api to master
     */
    WORKFLOW_EXECUTING_DATA_REQUEST,

    /**
     * update taskInstance's PID request
     */
    TASK_UPDATE_RUNTIME_MESSAGE,

    /**
     * update taskInstance's PID response ack, from master to worker
     */
    TASK_UPDATE_RUNTIME_MESSAGE_ACK,

    WORKFLOW_METRICS_CLEANUP,

    PAUSE_TASK_INSTANCE,
}

impl From<u8> for MessageType {
    fn from(v: u8) -> Self {
        match v {
            0 => MessageType::RESPONSE,
            1 => MessageType::GET_APP_ID_REQUEST,
            2 => MessageType::REMOVE_TAK_LOG_REQUEST,
            3 => MessageType::ROLL_VIEW_LOG_REQUEST,
            4 => MessageType::VIEW_WHOLE_LOG_REQUEST,
            5 => MessageType::GET_LOG_BYTES_REQUEST,
            6 => MessageType::TASK_EXECUTE_START,
            7 => MessageType::TASK_DISPATCH_REQUEST,
            8 => MessageType::TASK_DISPATCH_RESPONSE,
            9 => MessageType::TASK_EXECUTE_RUNNING_MESSAGE,
            10 => MessageType::TASK_EXECUTE_RUNNING_MESSAGE_ACK,
            11 => MessageType::TASK_EXECUTE_RESULT_MESSAGE,
            12 => MessageType::TASK_EXECUTE_RESULT_MESSAGE_ACK,
            13 => MessageType::TASK_KILL_REQUEST,
            14 => MessageType::TASK_SAVEPOINT_REQUEST,
            15 => MessageType::HEART_BEAT,
            16 => MessageType::PING,
            17 => MessageType::PONG,
            18 => MessageType::ALERT_SEND_REQUEST,
            19 => MessageType::WORKFLOW_HOST_CHANGE_REQUEST,
            20 => MessageType::STATE_EVENT_REQUEST,
            21 => MessageType::CACHE_EXPIRE,
            22 => MessageType::TASK_FORCE_STATE_EVENT_REQUEST,
            23 => MessageType::TASK_WAKEUP_EVENT_REQUEST,
            24 => MessageType::WORKFLOW_EXECUTING_DATA_REQUEST,
            25 => MessageType::TASK_UPDATE_RUNTIME_MESSAGE,
            26 => MessageType::TASK_UPDATE_RUNTIME_MESSAGE_ACK,
            27 => MessageType::WORKFLOW_METRICS_CLEANUP,
            28 => MessageType::PAUSE_TASK_INSTANCE,
            _ => MessageType::RESPONSE,
        }
    }
}
