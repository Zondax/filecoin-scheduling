use crate::{ClientToken, TaskRequirements};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(ClientToken, TaskRequirements),
    ListAllocations,
    WaitPreemptive(ClientToken),
    Release(ClientToken),
    ReleasePreemptive(ClientToken),
    Abort(u32),
    Monitoring,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum PreemptionResponse {
    Execute,
    Wait,
    Abort,
}
