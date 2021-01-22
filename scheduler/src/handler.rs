use crate::requests::SchedulerRequest;

pub trait Handler: Send + Sync + 'static {
    // TODO: Customize error type
    fn process_request(&self, request: SchedulerRequest);
}
