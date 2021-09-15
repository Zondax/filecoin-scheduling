use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

use crate::monitor::MonitorInfo;
use crate::{ClientToken, DeviceId, Pid, ResourceAlloc, Result, TaskRequirements};

#[derive(Serialize, Deserialize)]
pub enum RequestMethod {
    Schedule(ClientToken, TaskRequirements),
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

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<ResourceAlloc>>),
    SchedulerWaitPreemptive(Result<PreemptionResponse>),
    ListAllocations(Result<Vec<(DeviceId, u64)>>),
    Release,
    ReleasePreemptive,
    Abort(Result<()>),
    RemoveStalled(Result<()>),
    CheckService(Pid),
    Monitoring(std::result::Result<MonitorInfo, String>),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
