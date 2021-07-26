use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use chrono::Utc;
use parking_lot::RwLock;
use rust_gpu_tools::opencl::GPUSelector;
use tracing::{debug, error, instrument, trace, warn};

use crate::config::{Settings, Task};
use crate::handler::Handler;
use crate::monitor::{GpuResource, MonitorInfo, Task as MonitorTask};
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solver::{ResourceState, Resources, TaskState};
use crate::solvers::create_solver;
use crate::Error;
use common::{
    ClientToken, Devices, PreemptionResponse, RequestMethod, ResourceType, TaskId,
    TaskRequirements, TaskType,
};

// match all the devices that were assigned to task with type taskType
// returns None if there are not.
pub fn match_task_devices(
    task_type: Option<TaskType>,
    scheduler_settings: &[Task],
) -> Option<Vec<GPUSelector>> {
    let this_task = task_type?;
    for task in scheduler_settings {
        let devices = task.devices();
        if task.task_type() == this_task && !devices.is_empty() {
            return Some(devices);
        }
    }
    None
}

// compute whether a task is considered stalled
//
// if no taskType is provided then the task is valid if its `last_seen` is at least
// settings.min_wait_time seconds before now
//
// if a taskType is provided then the task exec time is fetched and used instead of
// settings.min_wait_time
pub fn task_is_stalled(
    last_seen: u64,
    task_type: Option<TaskType>,
    scheduler_settings: &Settings,
) -> bool {
    let min_wait_time = scheduler_settings.time_settings.min_wait_time;
    if task_type.is_none() {
        return Utc::now().timestamp() as u64 - min_wait_time > last_seen;
    }
    let this_task = task_type.unwrap();
    let time_of_task = scheduler_settings
        .tasks_settings
        .iter()
        .find(|task| task.task_type() == this_task)
        .map(|task| task.exec_time());
    if time_of_task.is_none() {
        //should not happen though....
        return Utc::now().timestamp() as u64 - min_wait_time > last_seen;
    }
    Utc::now().timestamp() as u64 - time_of_task.unwrap() > last_seen
}

#[derive(Debug)]
pub(crate) struct Scheduler {
    // Keep a cache of jobs on the system. each job_id has an associated job state
    // indicating the current iteration, and allocated resources and its requirements per resource
    tasks_state: RwLock<HashMap<TaskId, TaskState>>,

    // Sorted jobs to be executed.
    jobs_queue: RwLock<VecDeque<TaskId>>,

    devices: RwLock<Resources>,
    settings: Settings,
}

impl Scheduler {
    pub fn new(settings: Settings, devices: Devices) -> Self {
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
                        current_task: None,
                    },
                )
            })
            .collect::<HashMap<GPUSelector, ResourceState>>();
        let devices = RwLock::new(Resources(state));
        Self {
            tasks_state: RwLock::new(HashMap::new()),
            jobs_queue: RwLock::new(VecDeque::new()),
            devices,
            settings,
        }
    }

    #[instrument(level = "info", skip(requirements, self))]
    fn schedule(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
        job_context: String,
    ) -> SchedulerResponse {
        if requirements.req.is_empty() {
            error!("Schedule request with empty parameters");
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }
        // before continuing we need to check if this client.pid is already in the
        // jobs queue meaning there is a collision that we need to avoid
        // by ignoring the request until the previous job is done.
        if self.tasks_state.read().contains_key(&client.pid) {
            warn!(
                "Ignoring request - A client with the same id: {} is already in the queue ",
                client.pid
            );
            return SchedulerResponse::Schedule(Ok(None));
        }

        let restrictions =
            match_task_devices(requirements.task_type, &self.settings.tasks_settings);

        let resources = self.devices.read();

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if !resources.has_min_available_memory(&requirements) {
            return SchedulerResponse::Schedule(Ok(None));
        }

        let mut solver = create_solver(None);
        let (alloc, new_resources) = match solver.allocate_task(
            &resources,
            &requirements,
            &restrictions,
            &*self.tasks_state.read(),
        ) {
            Some(res) => res,
            _ => return SchedulerResponse::Schedule(Ok(None)), // Should not happen, we filtered lines before
        };
        // drop here just for updating the resources state
        drop(resources);

        {
            let mut dev = self.devices.write();
            dev.0 = new_resources;
        }

        let time: u64 = Utc::now().timestamp() as u64;

        // prepare the task
        let task_state = TaskState {
            requirements,
            allocation: alloc.clone(),
            last_seen: AtomicU64::new(time),
            aborted: AtomicBool::new(false),
            creation_time: time,
            context: job_context,
        };

        // Add the task to our list of jobs
        let mut state = self.tasks_state.write();
        state.insert(client.pid, task_state);

        // Update our plan
        let new_plan = match solver.solve_job_schedule(&*state, &self.settings) {
            Ok(plan) => {
                debug!("scheduler job_plan {:?}", plan);
                plan
            }
            Err(e) => return SchedulerResponse::Schedule(Err(Error::SolverOther(e.to_string()))),
        };

        drop(state);

        {
            *self.jobs_queue.write() = new_plan;
        }

        SchedulerResponse::Schedule(Ok(Some(alloc)))
    }

    fn log_stalled_jobs(&self) {
        let queue = self.jobs_queue.read();
        let state = self.tasks_state.read();
        for job_id in queue.iter() {
            if let Some(task) = state.get(job_id) {
                if task_is_stalled(
                    task.last_seen.load(Ordering::Relaxed),
                    task.requirements.task_type,
                    &self.settings,
                ) {
                    warn!("Process {}:{} is stalling!!", job_id, task.context);
                }
            }
        }
    }

    // this client has to wait if another is currently using the resource it shares
    fn wait_for_busy_resources(&self, client: &ClientToken) -> Result<bool, Error> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        let resources = self.devices.read();
        Ok(resources.has_busy_resources(current_task.allocation.devices.as_slice()))
    }

    // update the last_seen counter
    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn update_last_seen(&self, client: &ClientToken) -> Result<(), Error> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        // update the last_seen counter
        current_task
            .last_seen
            .store(Utc::now().timestamp() as u64, Ordering::Relaxed);
        Ok(())
    }

    //noinspection RsSelfConvention
    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn set_resource_as_busy(&self, client: ClientToken) {
        let state = self.tasks_state.read();
        if let Some(current_task) = state.get(&client.pid) {
            self.devices
                .write()
                .set_busy_resources(&current_task.allocation.devices, client.pid);
        }
    }

    // checks whether the job can continue or not depending on its position in the priority queue.
    // returns true if the job is at the top of the queue or among other jobs that share the same
    // resource. false if it has to wait
    #[instrument(level = "trace", skip(self), fields(pid = client))]
    fn check_priority_queue(&self, client: TaskId) -> Result<bool, Error> {
        let queue = self.jobs_queue.read();
        debug!("current job_plan {:?}", *queue);
        // check the job plan to see if the task is up-front the queue or not
        if let Some(job) = queue.front() {
            // return immediately if the task is at the front of the queue
            if *job == client {
                Ok(true)
            } else {
                let state = self.tasks_state.read();
                let current_task = state.get(&client).ok_or(Error::UnknownClient)?;

                // in this case we get an ordered queue based on the priority(highest to lowest) of the tasks that were assigned to the same
                // resource as client.
                let sub_queue = queue
                    .iter()
                    .filter(|id| {
                        // this unwrap wont panic we are traversing the current queue
                        let next_task = state.get(id).unwrap();
                        current_task.allocation.devices.iter().any(|dev_id| {
                            next_task.allocation.devices.iter().any(|id| dev_id == id)
                        })
                    })
                    .collect::<Vec<_>>();
                // this sub_queue will always contain at least one element
                // if current task is at the top means it does not have to wait.
                Ok(client == *sub_queue[0])
            }
        } else {
            warn!("Queue empty!");
            unreachable!();
        }
    }

    //is_task_from_client_aborted
    fn abort_client(&self, client: &ClientToken) -> Result<bool, Error> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        Ok(current_task.aborted.load(Ordering::Relaxed))
    }

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn wait_preemptive(&self, client: ClientToken) -> Result<PreemptionResponse, Error> {
        if self.abort_client(&client)? {
            return Ok(PreemptionResponse::Abort);
        }
        // update the last_seen counter
        self.update_last_seen(&client)?;
        self.log_stalled_jobs();

        // fast path the task's resource is being used by another task
        if self.wait_for_busy_resources(&client)? {
            return Ok(PreemptionResponse::Wait);
        }

        if self.check_priority_queue(client.pid)? {
            self.set_resource_as_busy(client);
            Ok(PreemptionResponse::Execute)
        } else {
            Ok(PreemptionResponse::Wait)
        }
    }

    // returns (device_id, available memory)
    fn list_allocations(&self) -> SchedulerResponse {
        let alloc = self
            .devices
            .read()
            .0
            .iter()
            .filter_map(|(i, device)| {
                if device.mem_usage() > 0 {
                    Some((*i, device.available_memory()))
                } else {
                    None
                }
            })
            .collect::<Vec<(GPUSelector, u64)>>();
        SchedulerResponse::ListAllocations(Ok(alloc))
    }

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn release(&self, client: ClientToken) {
        let task_state = { self.tasks_state.write().remove(&client.pid) };
        if let Some(state) = task_state {
            if let ResourceType::Gpu(ref m) = state.allocation.requirement.resource {
                self.devices
                    .write()
                    .free_memory(m, state.allocation.devices.as_slice());
            }
            (*self.jobs_queue.write()).retain(|pid| *pid != client.pid);
        } else {
            warn!("Task resources already released");
        }
    }

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn release_preemptive(&self, client: ClientToken) {
        let state = self.tasks_state.read();
        if let Some(current_task) = state.get(&client.pid) {
            self.devices
                .write()
                .unset_busy_resources(&current_task.allocation.devices, client.pid);
            debug!(
                "marking resource(s) as free {:?}",
                current_task.allocation.devices
            );
        } else {
            warn!("Task: {} is not in the queue - ignoring", client.pid);
        }
    }

    fn abort(&self, clients: Vec<TaskId>) -> Result<(), Error> {
        for client in clients.iter() {
            let state = self.tasks_state.read();
            let current_task = state.get(client).ok_or(Error::UnknownClient)?;
            warn!("aborting client: {} from: {}", client, current_task.context);
            current_task.aborted.store(true, Ordering::Relaxed);
        }
        Ok(())
    }

    // this function is experimental and might be removed in later versions of the
    // scheduler.
    fn remove_stalled(&self, clients: Vec<TaskId>) -> Result<(), Error> {
        for client in clients {
            let mut state = self.tasks_state.write();
            let task = state.get(&client).ok_or(Error::UnknownClient)?;
            if task_is_stalled(
                task.last_seen.load(Ordering::Relaxed),
                task.requirements.task_type,
                &self.settings,
            ) {
                trace!(
                    "task: {} from: {} is stalling, removing",
                    client,
                    task.context
                );

                let task = state.remove(&client).expect("Job in the state yet");
                // remove job from the priority queue
                (*self.jobs_queue.write()).retain(|pid| *pid != client);
                drop(state);
                // only the process with higher priority can mark the resource as free
                self.devices
                    .write()
                    .unset_busy_resources(&task.allocation.devices, client);
                if let ResourceType::Gpu(ref m) = task.allocation.requirement.resource {
                    self.devices
                        .write()
                        .free_memory(m, task.allocation.devices.as_slice());
                }
            } else {
                return Err(Error::JobNotStalling(client));
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    fn monitor(&self) -> Result<MonitorInfo, String> {
        let task_states = self.tasks_state.read();
        let resources = self.devices.read();
        let task_states = task_states
            .iter()
            .map(|(id, state)| {
                let last_seen = state.last_seen.load(Ordering::Relaxed);
                MonitorTask {
                    id: *id,
                    alloc: state.allocation.clone(),
                    task_type: state.requirements.task_type,
                    deadline: state.requirements.deadline,
                    last_seen,
                    stalled: task_is_stalled(
                        last_seen,
                        state.requirements.task_type,
                        &self.settings,
                    ),
                }
            })
            .collect::<Vec<_>>();
        let resources = resources
            .0
            .iter()
            .map(|(id, state)| GpuResource {
                device_id: *id,
                name: state.dev.name(),
                memory: state.dev.memory(),
                mem_usage: state.mem_usage,
                is_busy: state.is_busy(),
            })
            .collect::<Vec<_>>();
        Ok(MonitorInfo {
            task_states,
            resources,
            job_plan: self.jobs_queue.read().iter().copied().collect::<Vec<_>>(),
        })
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        // TODO: Analyze if spawning a thread is worth considering that doing so the handler's
        // Executor doesnt get blocked by this intensive operation
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(client, req, context) => self.schedule(client, req, context),
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
            RequestMethod::Abort(client_id) => SchedulerResponse::Abort(self.abort(client_id)),
            RequestMethod::RemoveStalled(client_id) => {
                SchedulerResponse::RemoveStalled(self.remove_stalled(client_id))
            }
            RequestMethod::Monitoring => SchedulerResponse::Monitoring(self.monitor()),
        };
        let _ = sender.send(response);
    }
}
