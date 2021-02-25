use common::{Error, RequestMethod, ResourceAlloc};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<Vec<ResourceAlloc>>, Error>),
    SchedulerWaitPreemptive(bool),
    ListAllocations(Vec<u32>),
    Release,
    ReleasePreemptive,
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
