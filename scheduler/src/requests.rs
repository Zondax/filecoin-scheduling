use common::{Error, RequestMethod, ResourceAlloc};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    // TODO: Meake the error a type and not a simple string
    Schedule(Result<ResourceAlloc, Error>),
    SchedulePreemptive(String),
    SchedulerWaitPreemptive(bool),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
