/**
 * type of task state
 */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum TaskStateType {
    /**
     * 0 waiting running
     * 1 running
     * 2 finish
     * 3 failed
     * 4 success
     */
    WAITTING,
    RUNNING,
    FINISH,
    FAILED,
    SUCCESS,
}

// impl TaskStateType{
//  pub fn  convert2ExecuteStatusIntArray(&self)->Vec<ExecutionStatus> {

//         match  self {
//              SUCCESS=>
//                 return  vec![]{ExecutionStatus.SUCCESS.ordinal()};
//              FAILED=>
//                 return new int[]{
//                         ExecutionStatus.FAILURE.ordinal(),
//                         ExecutionStatus.NEED_FAULT_TOLERANCE.ordinal()};
//             case FINISH:
//                 return new int[]{
//                         ExecutionStatus.PAUSE.ordinal(),
//                         ExecutionStatus.STOP.ordinal()
//                 };
//             case RUNNING:
//                 return new int[]{ExecutionStatus.SUBMITTED_SUCCESS.ordinal(),
//                         ExecutionStatus.DISPATCH.ordinal(),
//                         ExecutionStatus.RUNNING_EXECUTION.ordinal(),
//                         ExecutionStatus.DELAY_EXECUTION.ordinal(),
//                         ExecutionStatus.READY_PAUSE.ordinal(),
//                         ExecutionStatus.READY_STOP.ordinal()};
//             case WAITTING:
//                 return new int[]{
//                         ExecutionStatus.SUBMITTED_SUCCESS.ordinal()
//                 };
//             default:
//                 break;
//         }

//     }
// }
