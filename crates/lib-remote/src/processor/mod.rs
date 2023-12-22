pub trait RequestProcessor {
    fn process(&self);
}
trait MasterRpcProcessor: RequestProcessor {}

pub struct BaseLogProcessor;
pub struct WorkflowMetricsCleanUpProcessor;

impl RequestProcessor for BaseLogProcessor {
    fn process(&self) {
        println!("BaseLogProcessor");
    }
}
impl RequestProcessor for WorkflowMetricsCleanUpProcessor {
    fn process(&self) {
        println!("WorkflowMetricsCleanUpProcessor");
    }
}

pub fn exce(p: Box<dyn RequestProcessor>) {
    p.process();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor() {
        let processors = vec![
            Box::new(BaseLogProcessor) as Box<dyn RequestProcessor>,
            Box::new(WorkflowMetricsCleanUpProcessor) as Box<dyn RequestProcessor>,
        ];
        for p in processors {
            exce(p);
        }
    }
}
