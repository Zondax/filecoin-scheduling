use crate::client::Pid;
use serde::{Deserialize, Serialize};

use crate::{ClientToken, TaskRequirements};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(ClientToken, TaskRequirements, String),
    ListAllocations,
    WaitPreemptive(ClientToken),
    Release(ClientToken),
    ReleasePreemptive(ClientToken),
    Abort(Vec<Pid>),
    RemoveStalled(Vec<Pid>),
    CheckService,
    Monitoring,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum PreemptionResponse {
    Execute,
    Wait,
    Abort,
}
