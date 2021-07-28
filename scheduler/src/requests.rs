use futures::channel::oneshot;
use rust_gpu_tools::opencl::GPUSelector;
use serde::{Deserialize, Serialize};

use common::{Pid, PreemptionResponse, RequestMethod, ResourceAlloc};

use crate::monitor::MonitorInfo;
use crate::Error;

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<ResourceAlloc>, Error>),
    SchedulerWaitPreemptive(Result<PreemptionResponse, Error>),
    ListAllocations(Result<Vec<(GPUSelector, u64)>, Error>),
    Release,
    ReleasePreemptive,
    Abort(Result<(), Error>),
    RemoveStalled(Result<(), Error>),
    CheckService(Pid),
    Monitoring(Result<MonitorInfo, String>),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
