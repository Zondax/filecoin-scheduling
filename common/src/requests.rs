use serde::{Deserialize, Serialize};

use crate::{ClientToken, TaskRequirements};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(ClientToken, TaskRequirements, Option<String>),
    ListAllocations,
    WaitPreemptive(ClientToken),
    Release(ClientToken),
    ReleasePreemptive(ClientToken),
    Abort(Vec<u32>),
    RemoveStalled(Vec<u32>),
    Monitoring,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum PreemptionResponse {
    Execute,
    Wait,
    Abort,
}
