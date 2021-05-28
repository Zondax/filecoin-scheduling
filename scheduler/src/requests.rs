use crate::monitor::MonitorInfo;
use crate::Error;
use common::{PreemptionResponse, RequestMethod, ResourceAlloc};
use futures::channel::oneshot;
use rust_gpu_tools::opencl::DeviceUuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SchedulerResponse {
    Schedule(Result<Option<ResourceAlloc>, Error>),
    SchedulerWaitPreemptive(Result<PreemptionResponse, Error>),
    ListAllocations(Result<Vec<(DeviceUuid, u64)>, Error>),
    Release,
    ReleasePreemptive,
    Abort(Result<(), Error>),
    Monitoring(Result<MonitorInfo, String>),
}

pub struct SchedulerRequest {
    pub sender: oneshot::Sender<SchedulerResponse>,
    pub method: RequestMethod,
}
