use crate::requests::SchedulerRequest;

pub trait Handler: Send + Sync + 'static {
    fn process_request(&self, request: SchedulerRequest);
    fn maintenance(&self) {}
}
