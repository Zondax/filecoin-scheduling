use common::{Error, RequestMethod, ResourceAlloc};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<ResourceAlloc, Error>),
    SchedulePreemptive(String),
    ListAllocations(Vec<u32>),
    SchedulerWaitPreemptive(bool),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
