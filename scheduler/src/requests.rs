use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(String),
    SchedulePreemptive(String),
}

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    // TODO: Meake the error a type and not a simple string
    Schedule(Result<String, String>),
    SchedulePreemptive(String),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
