use crate::TaskRequirements;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(TaskRequirements),
    ListAllocations,
    SchedulePreemptive(String),
}
