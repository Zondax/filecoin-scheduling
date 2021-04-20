use chrono::Utc;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use crate::config::{Settings, Task};
use crate::handler::Handler;
use crate::monitor::{GpuResource, MonitorInfo, Task as MonitorTask};
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solver::{ResourceState, Resources, TaskState};
use crate::solvers::create_solver;
#[cfg(feature = "mip_solver")]
use crate::solvers::RequirementsMap;
use crate::Error;
use common::{
    ClientToken, PreemptionResponse, RequestMethod, ResourceType, TaskRequirements, TaskType,
};

pub fn match_task_devices(
    tasktype: Option<TaskType>,
    scheduler_settings: &[Task],
) -> Option<Vec<u64>> {
    let this_task = tasktype?;
    Some(
        scheduler_settings
            .iter()
            .filter(|task| task.get_task_type() == this_task)
            .map(|task| task.get_devices())
            .flatten()
            .collect::<Vec<u64>>(),
    )
}

pub fn task_is_stalled(
    last_seen: u64,
    tasktype: Option<TaskType>,
    scheduler_settings: &Settings,
) -> bool {
    let min_wait_time = scheduler_settings.time_settings.min_wait_time;
    if tasktype.is_none() {
        return Utc::now().timestamp() as u64 - min_wait_time > last_seen;
    }
    let this_task = tasktype.unwrap();
    let time_of_task = scheduler_settings
        .tasks_settings
        .iter()
        .find(|task| task.get_task_type() == this_task)
        .map(|task| task.get_task_exec_time());
    if time_of_task.is_none() {
        //should not happen though....
        return Utc::now().timestamp() as u64 - min_wait_time > last_seen;
    }
    Utc::now().timestamp() as u64 - time_of_task.unwrap() > last_seen
}

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
    settings: Settings,
}

impl Scheduler {
    pub fn new() -> Self {
        // TODO: modify this later
        let settings = Settings::new("/tmp/scheduler.toml").unwrap_or_default();
        let devices = common::list_devices();
        // Created a solver
        let state = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: Default::default(),
                        is_busy: Default::default(),
                    },
                )
            })
            .collect::<HashMap<u64, ResourceState>>();
        let devices = RwLock::new(Resources(state));
        Self {
            tasks_state: RwLock::new(HashMap::new()),
            jobs_queue: RwLock::new(VecDeque::new()),
            devices,
            settings,
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

        let restrictions =
            match_task_devices(requirements.task_type, &self.settings.tasks_settings);

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if !resources.has_min_available_memory(&requirements) {
            return SchedulerResponse::Schedule(Ok(None));
        }

        let mut solver = create_solver(None);
        // Try passing a mutable reference to resources
        let (alloc, new_resources) =
            match solver.allocate_task(&resources, &requirements, &restrictions) {
                Some(res) => res,
                _ => return SchedulerResponse::Schedule(Ok(None)), // Should not happen, we filtered lines before
            };

        resources.0 = new_resources;

        // Do not hold the lock anymore
        // allowing others to take it if needed
        drop(resources);

        let time: u64 = Utc::now().timestamp() as u64;

        // prepare the task
        let task_state = TaskState {
            requirements,
            current_iteration: 0,
            allocation: alloc.clone(),
            last_seen: AtomicU64::new(time),
        };

        // Add the task to our list of jobs
        let mut state = self.tasks_state.write().expect("Task state unwritable");
        state.insert(client.process_id(), task_state);

        // Update our plan
        let new_plan = match solver.solve_job_schedule(&*state, &self.settings) {
            Ok(plan) => plan,
            Err(e) => return SchedulerResponse::Schedule(Err(Error::SolverOther(e.to_string()))),
        };

        tracing::info!("scheduler job_plan {:?}", new_plan);
        {
            *self.jobs_queue.write().expect("Jobs queue unwritable") = new_plan;
        }

        SchedulerResponse::Schedule(Ok(Some(alloc)))
    }

    fn log_stalled_jobs(&self) {
        let queue = self.jobs_queue.read().unwrap();
        let state = self.tasks_state.read().unwrap();
        for job_id in queue.iter() {
            let task = state.get(&job_id).unwrap();
            if task_is_stalled(
                task.last_seen.load(Ordering::Relaxed),
                task.requirements.task_type,
                &self.settings,
            ) {
                tracing::warn!("Process {} is stalling!!", job_id);
            }
        }
    }

    fn wait_preemptive(&self, client: ClientToken) -> Result<PreemptionResponse, Error> {
        tracing::info!("scheduler: client {} wait preemtive", client.process_id());
        let state = self.tasks_state.read().unwrap();
        let current_task = state.get(&client.process_id()).ok_or(Error::RwError)?;
        current_task
            .last_seen
            .store(Utc::now().timestamp() as u64, Ordering::Relaxed);
        {
            let resources = self.devices.read().unwrap();
            if resources.has_busy_resources(&current_task.allocation.resource_id) {
                self.log_stalled_jobs();
                return Ok(PreemptionResponse::Wait); //client should sleep 2 seconds (LONG)
            }
        }
        let mut wait = false;
        {
            let queue = self.jobs_queue.read().unwrap();
            tracing::info!("scheduler job_plan {:?}", queue);
            if let Some(job) = queue.front() {
                if *job == client.process_id() {
                    //tracing::info!("client {} already upfront of the queue", *job);
                    wait = false;
                } else {
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
                }
            } else {
                tracing::warn!("Queue empty!");
                wait = false
            }
        }
        if !wait {
            let mut resources_write = self.devices.try_write().map_err(|_| Error::RwError)?;
            resources_write.set_busy_resources(&current_task.allocation.resource_id);
            Ok(PreemptionResponse::Execute)
        } else {
            let push_back: bool;
            {
                let queue = self.jobs_queue.read().unwrap();
                let front_task = queue.front().unwrap();
                let task = state.get(&front_task).unwrap();
                push_back = task_is_stalled(
                    task.last_seen.load(Ordering::Relaxed),
                    task.requirements.task_type,
                    &self.settings,
                );
            }
            if push_back {
                let mut queue_write = self.jobs_queue.try_write().map_err(|_| Error::RwError)?;
                let job = queue_write.pop_front().unwrap();
                queue_write.push_back(job);
                tracing::warn!("Pushing process {} to back!!", job);
            }
            Ok(PreemptionResponse::Wait)
        }
    }

    // returns (device_id, available memory)
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
            .collect::<Vec<(u64, u64)>>();
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
            let mut solver = create_solver(None);
            // Update our plan
            let state = self.tasks_state.read().unwrap();
            if let Ok(plan) = solver.solve_job_schedule(&*state, &self.settings) {
                *self.jobs_queue.write().unwrap() = plan
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

    fn monitor(&self) -> Result<MonitorInfo, String> {
        let task_states = self.tasks_state.read().map_err(|e| e.to_string())?;
        let task_states = task_states
            .iter()
            .map(|(id, state)| MonitorTask {
                id: *id,
                alloc: state.allocation.clone(),
                task_type: state.requirements.task_type,
                deadline: state.requirements.deadline,
                last_seen: state.last_seen.load(Ordering::Relaxed),
            })
            .collect::<Vec<_>>();
        let resources = self.devices.read().map_err(|e| e.to_string())?;
        let resources = resources
            .0
            .iter()
            .map(|(id, state)| GpuResource {
                device_id: *id,
                name: state.dev.name(),
                mem_usage: state.mem_usage,
                is_busy: state.is_busy,
            })
            .collect::<Vec<_>>();
        Ok(MonitorInfo {
            task_states,
            resources,
        })
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
            RequestMethod::Abort(_client_id) => {
                //TODO: Implement abort logic
                SchedulerResponse::Abort
            }
            RequestMethod::Monitoring => SchedulerResponse::Monitoring(self.monitor()),
        };
        let _ = sender.send(response);
    }
}
