use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use common::{ClientToken, Devices, Error, RequestMethod, ResourceAlloc, TaskRequirements};

pub(crate) struct Scheduler {
    _state_path: PathBuf,
    _task_queue: VecDeque<SchedulerRequest>,
    // List the resources and in the case the resource is being used, the id of the client using it
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

    fn schedule(&self, client: ClientToken, requirements: TaskRequirements) -> SchedulerResponse {
        if requirements.req.is_empty() {
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }

        // Here we call our MIP solver that returns the bus_id of the GPU
        // for now we use a default value;
        let bus_id: u32 = Default::default();
        let alloc = ResourceAlloc {
            // For now just use the first req
            resource: requirements.req[0].clone(),
            resource_id: bus_id,
        };

        // Update the scheduler state so that, the resource identified by bus_id gets updated with the
        // process ID to whom it is assigned to.
        if let Some(v) = self
            .state
            .write()
            .expect("Should be called once")
            .get_mut(&bus_id)
        {
            // Only free resources can be assigned
            if !v.is_some() {
                v.replace(client.process_id());
                SchedulerResponse::Schedule(Ok(Some(alloc)))
            } else {
                // TODO: The resource is being used.
                // should this be reported as an error to the client?
                SchedulerResponse::Schedule(Ok(None))
            }
        } else {
            // The resource doesnt exists or the system doesnt have GPU devices
            // we return None here
            SchedulerResponse::Schedule(Ok(None))
        }
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

    fn release(&self, alloc: ResourceAlloc) {
        if let Some(state) = self
            .state
            .write()
            .expect("Should be called once")
            .get_mut(&alloc.resource_id)
        {
            state.take();
        }
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(client, req) => self.schedule(client, req),
            RequestMethod::ListAllocations => self.list_allocations(),
            RequestMethod::WaitPreemptive(_client, _timeout) => {
                SchedulerResponse::SchedulerWaitPreemptive(true)
            }
            RequestMethod::Release(alloc) => {
                self.release(alloc);
                SchedulerResponse::Release
            }
        };
        let _ = sender.send(response);
    }
}
