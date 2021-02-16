use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use common::{Device, Devices, Error, RequestMethod, ResourceAlloc, TaskRequirements};

pub(crate) struct Scheduler {
    _state_path: PathBuf,
    _task_queue: VecDeque<SchedulerRequest>,
    //State where the device with bus_id is associated to an optional process that is using the
    //resource, If the process is None means that the resource is not in used
    state: RwLock<HashMap<u32, Option<u32>>>,
    _devices: Devices,
}

impl Scheduler {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        let _task_queue = VecDeque::new();
        let _devices = common::list_devices();
        let state = _devices
            .gpu_devices()
            .iter()
            .filter_map(|dev| {
                if let Some(id) = dev.bus_id() {
                    Some((id, None))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, Option<u32>>>();
        Self {
            _task_queue,
            _state_path: path.into(),
            state: RwLock::new(state),
            _devices,
        }
    }

    fn schedule(&self, requirements: TaskRequirements) -> SchedulerResponse {
        if requirements.req.is_empty() {
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }
        let alloc = ResourceAlloc {
            // For now just use the first req
            resource: requirements.req[0].clone(),
            resource_id: 0,
        };
        SchedulerResponse::Schedule(Ok(alloc))
    }

    fn list_allocations(&self) -> SchedulerResponse {
        SchedulerResponse::ListAllocations(
            self.state
                .read()
                .expect("Already locked by this thread")
                .iter()
                .filter_map(|(k, v)| if v.is_some() { Some(*k) } else { None })
                .collect::<Vec<u32>>(),
        )
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(s) => self.schedule(s),
            RequestMethod::SchedulePreemptive(s) => SchedulerResponse::SchedulePreemptive(s),
            RequestMethod::ListAllocations => self.list_allocations(),
        };
        let _ = sender.send(response);
    }
}
