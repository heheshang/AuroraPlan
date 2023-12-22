#[allow(non_camel_case_types)]
pub enum MessageType {
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
