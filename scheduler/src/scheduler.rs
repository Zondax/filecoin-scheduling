use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solvers::RequirementsMap;
use crate::solvers::{create_solver, JobAllocation};
use common::{
    ClientToken, Devices, Error, RequestMethod, ResourceAlloc, ResourceType, TaskRequirements,
};

#[derive(Debug)]
pub(crate) struct Scheduler {
    _state_path: PathBuf,
    // List the resources and in the case the resource is being used, the id of the client using it
    state: RwLock<HashMap<u32, Option<JobAllocation>>>,
    devices: Devices,
}

impl Scheduler {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let devices = common::list_devices();
        // Created a solver
        let state = devices
            .gpu_devices()
            .iter()
            .filter_map(|dev| {
                if let Some(id) = dev.bus_id() {
                    Some((id, None))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, Option<JobAllocation>>>();
        Self {
            _state_path: path.into(),
            state: RwLock::new(state),
            devices,
        }
    }

    #[tracing::instrument(level = "info", skip(requirements, self))]
    fn schedule(&self, client: ClientToken, requirements: TaskRequirements) -> SchedulerResponse {
        if requirements.req.is_empty() {
            tracing::error!("Schedule request with empty parameters");
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }

        let mut num_gpu = 0usize;
        let mut num_mem = 0usize;

        let state = self.state.read().expect("read state panics");
        for req in requirements.req.iter() {
            match req.resource {
                ResourceType::Gpu => {
                    num_gpu += req.quantity;
                }
                ResourceType::GpuMemory(_) => {
                    num_mem += req.quantity;
                } // Later we will need to track the total amount of memory
                _ => unreachable!(), // TODO: evaluate this case in a real escenario
            }
        }

        let resources = self
            .devices
            .gpu_devices()
            .iter()
            .filter_map(|dev| {
                let bus_id = dev.bus_id().expect("Gpu device without a valid bus_id");
                // TODO: A check should be added to verify that this resource meet the requirements ?
                if state
                    .get(&bus_id)
                    .expect("State is built from devices")
                    .is_none()
                {
                    Some(bus_id)
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>();

        drop(state);

        let total_devices = num_gpu + num_mem;
        // If there are not enough available devices for the client requirements
        // just notify this to the client by returning None
        if resources.len() < total_devices {
            return SchedulerResponse::Schedule(Ok(None));
        }

        // Expensive copy?
        let task_req_copy = requirements.req.clone();
        let wrapper = RequirementsMap {
            reqs: requirements,
            resources,
            job_id: client.process_id() as usize,
            preemptive: None,
            has_started: None,
        };

        let mut solver = match create_solver(None) {
            Ok(solver) => solver,
            Err(e) => return SchedulerResponse::Schedule(Err(Error::Solver(e.to_string()))),
        };

        let job_plan = match solver.solve_job_schedule(wrapper.into(), 0, 0, None) {
            Ok(plan) => plan,
            Err(e) => {
                tracing::error!("Solver error: {:?}", e);
                return SchedulerResponse::Schedule(Err(Error::Solver(e.to_string())));
            }
        };

        let mut alloc = vec![];

        let mut state_writer = self.state.write().expect("Write called multiple times");
        // Get at least the number of requested devices
        for (i, plan) in job_plan.plan.iter().enumerate() {
            if let Some(dev) = state_writer.get_mut(&(plan.machine as _)) {
                if dev.is_none() {
                    alloc.push(ResourceAlloc {
                        resource: task_req_copy[i].clone(),
                        resource_id: plan.machine as _,
                    });
                    dev.replace(plan.clone());
                } else {
                    // TODO: The resource is being used.
                    // should this be reported as an error to the client?
                    return SchedulerResponse::Schedule(Ok(None));
                }
            } else {
                return SchedulerResponse::Schedule(Err(Error::Other(
                    "Unexpected error updating scheduler state".to_string(),
                )));
            };
        }

        if alloc.len() >= total_devices {
            SchedulerResponse::Schedule(Ok(Some(alloc)))
        } else {
            tracing::error!(
                "Client: {} - requested {} - not devices available",
                client.process_id(),
                total_devices
            );
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

    fn release(&self, alloc: Vec<ResourceAlloc>) {
        for resource in alloc.iter() {
            if let Some(state) = self
                .state
                .write()
                .expect("Should be called once")
                .get_mut(&resource.resource_id)
            {
                state.take();
            }
        }
    }

    fn release_preemptive(&self, _alloc: Vec<ResourceAlloc>) {
        //TODO
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        // TODO: Analize if spawning a thread is worth considering that doing so the handler
        // executer doesnt get blocked by this intensive operation
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(client, req) => self.schedule(client, req),
            RequestMethod::ListAllocations => self.list_allocations(),
            RequestMethod::WaitPreemptive(_client, _timeout) => {
                SchedulerResponse::SchedulerWaitPreemptive(false)
            }
            RequestMethod::Release(alloc) => {
                self.release(alloc);
                SchedulerResponse::Release
            }
            RequestMethod::ReleasePreemptive(alloc) => {
                self.release_preemptive(alloc);
                SchedulerResponse::ReleasePreemptive
            }
        };
        let _ = sender.send(response);
    }
}
