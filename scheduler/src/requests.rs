use futures::channel::oneshot;
use rust_gpu_tools::opencl::GPUSelector;
use serde::{Deserialize, Serialize};

use common::{PreemptionResponse, RequestMethod, ResourceAlloc};

use crate::Error;
use crate::monitor::MonitorInfo;

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<ResourceAlloc>, Error>),
    SchedulerWaitPreemptive(Result<PreemptionResponse, Error>),
    ListAllocations(Result<Vec<(GPUSelector, u64)>, Error>),
    Release,
    ReleasePreemptive,
    Abort(Result<(), Error>),
    RemoveStalled(Result<(), Error>),
    Monitoring(Result<MonitorInfo, String>),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
