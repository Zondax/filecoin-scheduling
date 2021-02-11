use crate::TaskRequirements;
use serde::{Deserialize, Serialize};
use crate::ClientToken;

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(TaskRequirements),
    SchedulePreemptive(String),
    WaitPreemptive(ClientToken, std::time::Duration),
}
