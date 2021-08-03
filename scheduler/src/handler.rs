use crate::requests::SchedulerRequest;

pub trait Handler: Send + Sync + 'static {
    fn process_request(&self, request: SchedulerRequest);

    // Perform  a maintenance iteration on this handler instance.
    // returns True if the maintenance cycle should continue, otherwise it will be closed
    // be closed
    fn maintenance(&self) -> bool {
        false
    }
}
