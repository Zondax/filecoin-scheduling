use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

use common::{DeviceId, Pid, PreemptionResponse, RequestMethod, ResourceAlloc};

use crate::monitor::MonitorInfo;
use crate::Result;

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
