use crate::monitor::MonitorInfo;
use crate::Error;
use common::{RequestMethod, ResourceAlloc};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<ResourceAlloc>, Error>),
    SchedulerWaitPreemptive(bool),
    ListAllocations(Result<Vec<(u64, u64)>, Error>),
    Release,
    ReleasePreemptive,
    Abort,
    Monitoring(Result<MonitorInfo, String>),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
