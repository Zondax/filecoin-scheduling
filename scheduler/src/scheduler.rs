use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solver::{ResourceState, Resources, TaskState};
use crate::solvers::create_solver;
#[cfg(feature = "mip_solver")]
use crate::solvers::RequirementsMap;
use common::{ClientToken, Error, RequestMethod, ResourceType, TaskRequirements};

#[derive(Debug)]
pub(crate) struct Scheduler {
    _state_path: PathBuf,
    // Keep a cache of jobs on the system. each job_id has associated a job state
    // indicating the current iteration, and allocated resources and its requirements per resource
    tasks_state: RwLock<HashMap<u32, TaskState>>,
    // A simple priority queue indexed by the resource_id, each resource could be assigned to
    // different jobs. being the job at the up-front the current "owner". The priority queue is
    // done by comparing deadlines

    // Sorted jobs to be executed.
    jobs_queue: RwLock<VecDeque<u32>>,

    devices: RwLock<Resources>,
}

impl Scheduler {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let devices = common::list_devices();
        // Created a solver
        let state = devices
            .gpu_devices()
            .iter()
            .map(|dev| ResourceState {
                dev: dev.clone(),
                mem_usage: 0,
                is_exclusive: devices.exclusive_gpus().iter().any(|&i| i==dev.bus_id()),
            })
            .collect::<Vec<ResourceState>>();
        let devices = RwLock::new(Resources(state));
        Self {
            _state_path: path.into(),
            tasks_state: RwLock::new(HashMap::new()),
            jobs_queue: RwLock::new(VecDeque::new()),
            devices,
        }
    }

    #[tracing::instrument(level = "info", skip(requirements, self))]
    fn schedule(&self, client: ClientToken, requirements: TaskRequirements) -> SchedulerResponse {
        if requirements.req.is_empty() {
            tracing::error!("Schedule request with empty parameters");
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }

        let mut resources = if let Ok(resc) = self.devices.write() {
            resc
        } else {
            return SchedulerResponse::Schedule(Err(Error::Other(
                "Can not read resources".to_string(),
            )));
        };

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if resources.available_memory(requirements.exclusive) < requirements.minimal_resource_usage() {
            return SchedulerResponse::Schedule(Ok(None));
        }

        let mut solver = match create_solver(None) {
            Ok(solver) => solver,
            Err(e) => return SchedulerResponse::Schedule(Err(Error::Solver(e.to_string()))),
        };

        let (alloc, new_resources) = match solver.allocate_task(&resources, &requirements) {
            Some(res) => res,
            _ => return SchedulerResponse::Schedule(Ok(None)), // Should not happen, we filtered lines before
        };

        resources.0 = new_resources;

        // prepare the task
        let task_state = TaskState {
            requirements,
            current_iteration: 0,
            allocation: alloc.clone(),
        };

        // Add the task to our list of jobs
        let mut state = self.tasks_state.write().expect("Task state unwritable");
        state.insert(client.process_id(), task_state);

        // Update our plan
        let new_plan = match solver.solve_job_schedule(&*state) {
            Ok(plan) => plan,
            Err(e) => return SchedulerResponse::Schedule(Err(Error::Solver(e.to_string()))),
        };

        tracing::info!("scheduler job_plan {:?}", new_plan);
        {
            *self.jobs_queue.write().expect("Jobs queue unwritable") = new_plan;
        }

        SchedulerResponse::Schedule(Ok(Some(alloc)))
    }

    fn wait_preemptive(&self, client: ClientToken) -> bool {
        let queue = self.jobs_queue.read().unwrap();
        if let Some(job) = queue.front() {
            if *job == client.process_id() {
                //tracing::info!("client {} already upfront of the queue", *job);
                false
            } else {
                //Checks if this task needs to wait for its turn as indicated in the plan
                //may be the resources are its and can continue immediately
                let mut wait = false;
                let state = self.tasks_state.read().unwrap();
                let current_task = state.get(&client.process_id()).unwrap();
                // This is expensive, we can do better by havving an associated table between
                // resources and tasks using it.
                for job_id in queue.iter().filter(|id| **id != client.process_id()) {
                    let next_task = state.get(job_id).unwrap();
                    wait = current_task.allocation.resource_id.iter().any(|dev_id| {
                        next_task
                            .allocation
                            .resource_id
                            .iter()
                            .any(|id| dev_id == id)
                    });
                    if wait {
                        break;
                    }
                }
                wait
            }
        } else {
            tracing::warn!("Queue empty!");
            false
        }
    }

    fn list_allocations(&self) -> SchedulerResponse {
        SchedulerResponse::ListAllocations(vec![])
    }

    fn release(&self, client: ClientToken) {
        let task_state = self
            .tasks_state
            .write()
            .unwrap()
            .remove(&client.process_id());
        if let Some(state) = task_state {
            if let ResourceType::Gpu(ref m) = state.allocation.requirement.resource {
                self.devices
                    .write()
                    .unwrap()
                    .free_memory(m, &state.allocation.resource_id);
            }
            if let Ok(mut solver) = create_solver(None) {
                // Update our plan
                let state = self.tasks_state.read().unwrap();
                *self.jobs_queue.write().unwrap() = match solver.solve_job_schedule(&*state) {
                    Ok(plan) => plan,
                    _ => return,
                };
            }
        } else {
            tracing::warn!("Task resources already released");
        }
    }

    fn release_preemptive(&self, _client: ClientToken) {}
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        // TODO: Analize if spawning a thread is worth considering that doing so the handler
        // executer doesnt get blocked by this intensive operation
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(client, req) => self.schedule(client, req),
            RequestMethod::ListAllocations => self.list_allocations(),
            RequestMethod::WaitPreemptive(client) => {
                SchedulerResponse::SchedulerWaitPreemptive(self.wait_preemptive(client))
            }
            RequestMethod::Release(client) => {
                self.release(client);
                SchedulerResponse::Release
            }
            RequestMethod::ReleasePreemptive(client) => {
                self.release_preemptive(client);
                SchedulerResponse::ReleasePreemptive
            }
        };
        let _ = sender.send(response);
    }
}
