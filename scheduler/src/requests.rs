use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(String),
    SchedulePreemptive(String),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(String),
    SchedulePreemptive(String),
}
