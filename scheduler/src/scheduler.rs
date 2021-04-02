use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::RwLock;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solver::{ResourceState, Resources, TaskState};
use crate::solvers::create_solver;
#[cfg(feature = "mip_solver")]
use crate::solvers::RequirementsMap;
use crate::Error;
use common::{ClientToken, RequestMethod, ResourceType, TaskRequirements};

#[derive(Debug)]
pub(crate) struct Scheduler {
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
    pub fn new() -> Self {
        let devices = common::list_devices();
        // Created a solver
        let state = devices
            .gpu_devices()
            .iter()
            .enumerate()
            .map(|(key, dev)| {
                (
                    key,
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: Default::default(),
                        is_busy: Default::default(),
                        is_exclusive: devices
                            .exclusive_gpus()
                            .iter()
                            .any(|&i| i == dev.device_id()),
                    },
                )
            })
            .collect::<HashMap<usize, ResourceState>>();
        let devices = RwLock::new(Resources(state));
        Self {
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

        let mut resources = if let Ok(resc) = self.devices.try_write() {
            resc
        } else {
            // TODO: Is this an error?
            return SchedulerResponse::Schedule(Err(Error::RwError));
        };

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if !resources.has_min_available_memory(&requirements) {
            return SchedulerResponse::Schedule(Ok(None));
        }

        let mut solver = match create_solver(None) {
            Ok(solver) => solver,
            Err(_) => return SchedulerResponse::Schedule(Err(Error::NoSolver)),
        };

        // Try passing a mutable reference to resources
        let (alloc, new_resources) = match solver.allocate_task(&resources, &requirements) {
            Some(res) => res,
            _ => return SchedulerResponse::Schedule(Ok(None)), // Should not happen, we filtered lines before
        };

        resources.0 = new_resources;

        // Do not hold the lock anymore
        // allowing others to take it if needed
        drop(resources);

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
            Err(e) => return SchedulerResponse::Schedule(Err(Error::SolverOther(e.to_string()))),
        };

        tracing::info!("scheduler job_plan {:?}", new_plan);
        {
            *self.jobs_queue.write().expect("Jobs queue unwritable") = new_plan;
        }

        SchedulerResponse::Schedule(Ok(Some(alloc)))
    }

    fn wait_preemptive(&self, client: ClientToken) -> bool {
        tracing::info!("scheduler: client {} wait preemtive", client.process_id());

        let queue = self.jobs_queue.read().unwrap();
        let state = self.tasks_state.read().unwrap();
        let current_task = if let Some(task) = state.get(&client.process_id()) {
            task
        } else {
            // Task that is not in our job_queue is asking for preemption
            // This is an error or just return true??
            return true;
        };
        {
            let resources = self.devices.read().unwrap();
            if resources.has_busy_resources(&current_task.allocation.resource_id) {
                return true; //client should sleep 2 seconds (LONG)
            }
        }
        if let Some(job) = queue.front() {
            if *job == client.process_id() {
                //tracing::info!("client {} already upfront of the queue", *job);
                let devwrite = self.devices.try_write();
                if devwrite.is_err() {
                    return true; //client should sleep short
                }
                let mut resources_write = devwrite.unwrap();
                resources_write.set_busy_resources(&current_task.allocation.resource_id);
                false
            } else {
                let mut wait = false;
                //Checks if this task needs to wait for its turn as indicated in the plan
                //may be the resources are its and can continue immediately
                // This is expensive, we can do better by havving an associated table between
                // resources and tasks using it.
                for job_id in queue.iter().take_while(|id| **id != client.process_id()) {
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
                if !wait {
                    let devwrite = self.devices.try_write();
                    if devwrite.is_err() {
                        return true;
                    }
                    let mut resources_write = devwrite.unwrap();
                    resources_write.set_busy_resources(&current_task.allocation.resource_id);
                }
                wait
            }
        } else {
            tracing::warn!("Queue empty!");
            false
        }
    }

    fn list_allocations(&self) -> SchedulerResponse {
        let alloc = self
            .devices
            .read()
            .unwrap()
            .0
            .iter()
            .filter_map(|(i, device)| {
                if device.mem_usage() > 0 {
                    Some((*i, device.available_memory()))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, u64)>>();
        SchedulerResponse::ListAllocations(Ok(alloc))
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
                match solver.solve_job_schedule(&*state) {
                    // Solve first then, assign the solution to our queue
                    // by doing so we do not hold the lock much time allowing other process to
                    // read/write in the meantime
                    Ok(plan) => *self.jobs_queue.write().unwrap() = plan,
                    _ => return,
                };
            }
        } else {
            tracing::warn!("Task resources already released");
        }
    }

    fn release_preemptive(&self, client: ClientToken) {
        tracing::info!("release preemtive client {}", client.process_id());
        let state = self.tasks_state.read().unwrap();
        if let Some(current_task) = state.get(&client.process_id()) {
            let mut resources_write = self.devices.write().unwrap();
            resources_write.unset_busy_resources(&current_task.allocation.resource_id);
        }
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
