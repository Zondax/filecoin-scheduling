use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(String),
    SchedulePreemptive(String),
}

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(String),
    SchedulePreemptive(String),
}
