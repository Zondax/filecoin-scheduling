use crate::{ClientToken, ResourceAlloc, TaskRequirements};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(ClientToken, TaskRequirements),
    ListAllocations,
    WaitPreemptive(ClientToken, std::time::Duration),
    Release(Vec<ResourceAlloc>),
    ReleasePreemptive(Vec<ResourceAlloc>),
}
