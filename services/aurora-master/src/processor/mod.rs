use async_trait::async_trait;
// use enum_dispatch::enum_dispatch;
use lib_remote::cmd::{message::Message, message_type::MessageType};
use tracing::info;
#[allow(non_camel_case_types)]
// #[enum_dispatch(MasterRpcProcessor)]
#[derive(Debug)]
pub enum MasterMessageType {
    CACHE_EXPIRE(CacheProcessor),
    TASK_DISPATCH_REQUEST(MasterTaskDispatchProcessor),
    TASK_KILL_REQUEST(MasterTaskKillProcessor),
    PAUSE_TASK_INSTANCE(MasterTaskPauseProcessor),
    STATE_EVENT_REQUEST(StateEventProcessor),
    TASK_EXECUTE_RESULT_MESSAGE(TaskExecuteResultProcessor),
    TASK_EXECUTE_RUNNING_MESSAGE(TaskExecuteRunningProcessor),
    TASK_EXECUTE_START(TaskExecuteStartProcessor),
    TASK_FORCE_STATE_EVENT_REQUEST(TaskForceStartProcessor),
    RESPONSE(TaskKillResponseProcessor),
    TASK_UPDATE_RUNTIME_MESSAGE(TaskUpdateRuntimeProcessor),
    TASK_WAKEUP_EVENT_REQUEST(TaskWakeupProcessor),
    WORKFLOW_EXECUTING_DATA_REQUEST(WorkflowExecutingDataRequestProcessor),
    WORKFLOW_METRICS_CLEANUP(WorkflowMetricsCleanUpProcessor),
}
impl MasterMessageType {
    pub async fn process(&self, msg: &Message) {
        match self {
            MasterMessageType::CACHE_EXPIRE(processor) => processor.process(msg).await,
            MasterMessageType::TASK_DISPATCH_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::TASK_KILL_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::PAUSE_TASK_INSTANCE(processor) => processor.process(msg).await,
            MasterMessageType::STATE_EVENT_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::TASK_EXECUTE_RESULT_MESSAGE(processor) => processor.process(msg).await,
            MasterMessageType::TASK_EXECUTE_RUNNING_MESSAGE(processor) => processor.process(msg).await,
            MasterMessageType::TASK_EXECUTE_START(processor) => processor.process(msg).await,
            MasterMessageType::TASK_FORCE_STATE_EVENT_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::RESPONSE(processor) => processor.process(msg).await,
            MasterMessageType::TASK_UPDATE_RUNTIME_MESSAGE(processor) => processor.process(msg).await,
            MasterMessageType::TASK_WAKEUP_EVENT_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::WORKFLOW_EXECUTING_DATA_REQUEST(processor) => processor.process(msg).await,
            MasterMessageType::WORKFLOW_METRICS_CLEANUP(processor) => processor.process(msg).await,
        }
    }
}

impl From<MessageType> for MasterMessageType {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::CACHE_EXPIRE => MasterMessageType::CACHE_EXPIRE(CacheProcessor),
            MessageType::TASK_DISPATCH_REQUEST => MasterMessageType::TASK_DISPATCH_REQUEST(MasterTaskDispatchProcessor),
            MessageType::TASK_KILL_REQUEST => MasterMessageType::TASK_KILL_REQUEST(MasterTaskKillProcessor),
            MessageType::PAUSE_TASK_INSTANCE => MasterMessageType::PAUSE_TASK_INSTANCE(MasterTaskPauseProcessor),
            MessageType::STATE_EVENT_REQUEST => MasterMessageType::STATE_EVENT_REQUEST(StateEventProcessor),
            MessageType::TASK_EXECUTE_RESULT_MESSAGE => {
                MasterMessageType::TASK_EXECUTE_RESULT_MESSAGE(TaskExecuteResultProcessor)
            }
            MessageType::TASK_EXECUTE_RUNNING_MESSAGE => {
                MasterMessageType::TASK_EXECUTE_RUNNING_MESSAGE(TaskExecuteRunningProcessor)
            }
            MessageType::TASK_EXECUTE_START => MasterMessageType::TASK_EXECUTE_START(TaskExecuteStartProcessor),
            MessageType::TASK_FORCE_STATE_EVENT_REQUEST => {
                MasterMessageType::TASK_FORCE_STATE_EVENT_REQUEST(TaskForceStartProcessor)
            }
            MessageType::RESPONSE => MasterMessageType::RESPONSE(TaskKillResponseProcessor),
            MessageType::TASK_UPDATE_RUNTIME_MESSAGE => {
                MasterMessageType::TASK_UPDATE_RUNTIME_MESSAGE(TaskUpdateRuntimeProcessor)
            }
            MessageType::TASK_WAKEUP_EVENT_REQUEST => MasterMessageType::TASK_WAKEUP_EVENT_REQUEST(TaskWakeupProcessor),
            MessageType::WORKFLOW_EXECUTING_DATA_REQUEST => {
                MasterMessageType::WORKFLOW_EXECUTING_DATA_REQUEST(WorkflowExecutingDataRequestProcessor)
            }
            MessageType::WORKFLOW_METRICS_CLEANUP => {
                MasterMessageType::WORKFLOW_METRICS_CLEANUP(WorkflowMetricsCleanUpProcessor)
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct CacheProcessor;
#[async_trait]
impl MasterRpcProcessor for CacheProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
        info!("CacheProcessor");
    }
}

#[derive(Debug)]
pub struct MasterTaskDispatchProcessor;
#[async_trait]
impl MasterRpcProcessor for MasterTaskDispatchProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}

#[derive(Debug)]
pub struct MasterTaskKillProcessor;
#[async_trait]
impl MasterRpcProcessor for MasterTaskKillProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}

#[derive(Debug)]
pub struct MasterTaskPauseProcessor;
#[async_trait]
impl MasterRpcProcessor for MasterTaskPauseProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}

#[derive(Debug)]
pub struct StateEventProcessor;
#[async_trait]
impl MasterRpcProcessor for StateEventProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskExecuteResultProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskExecuteResultProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskExecuteRunningProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskExecuteRunningProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskExecuteStartProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskExecuteStartProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskForceStartProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskForceStartProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskKillResponseProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskKillResponseProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskUpdateRuntimeProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskUpdateRuntimeProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct TaskWakeupProcessor;
#[async_trait]
impl MasterRpcProcessor for TaskWakeupProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct WorkflowExecutingDataRequestProcessor;
#[async_trait]
impl MasterRpcProcessor for WorkflowExecutingDataRequestProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}
#[derive(Debug)]
pub struct WorkflowMetricsCleanUpProcessor;
#[async_trait]
impl MasterRpcProcessor for WorkflowMetricsCleanUpProcessor {
    async fn process(&self, msg: &Message) {
        info!("{:?}", msg);
    }
}

// #[enum_dispatch]
#[async_trait]
pub trait MasterRpcProcessor: Send + Sync {
    async fn process(&self, msg: &Message);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() {
        let storages: Vec<MasterMessageType> = vec![MasterMessageType::CACHE_EXPIRE(CacheProcessor)];
        storages[0].process(&Message::default()).await;
    }
}
