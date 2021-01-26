use crate::requests::SchedulerRequest;

pub(crate) trait Handler: Send + Sync + 'static {
    // TODO: Customize error type
    fn process_request(&self, request: SchedulerRequest);
}
