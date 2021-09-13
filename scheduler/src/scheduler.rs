use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use sysinfo::{System, SystemExt};

use chrono::Utc;
use crossbeam::channel::Sender;
use parking_lot::{Mutex, RwLock};
use std::time::Instant;
use tracing::{debug, error, instrument, warn};

use crate::config::{Settings, Task};
use crate::db::Database;
use crate::handler::Handler;
use crate::monitor::{GpuResource, MonitorInfo, Task as MonitorTask};
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::solver::{ResourceState, Resources, TaskState};
use crate::solvers::create_solver;
use crate::{
    ClientToken, DeviceId, Devices, Pid, PreemptionResponse, RequestMethod, ResourceAlloc,
    ResourceType, TaskRequirements, TaskType,
};
use crate::{Error, Result};

// match all the devices that were assigned to task with type taskType
// returns None if there are not.
pub fn match_task_devices(
    task_type: Option<TaskType>,
    scheduler_settings: &[Task],
) -> Option<Vec<DeviceId>> {
    let this_task = task_type?;
    for task in scheduler_settings {
        let devices = task.devices();
        if task.task_type() == this_task && !devices.is_empty() {
            return Some(devices);
        }
    }
    None
}

/// compute whether a task is considered stalled
///
/// using the value of [Settings::min_wait_time] seconds before now
///
/// if [Settings::max_wait_time] is set, this function will check if the
/// stalled task should be removed regardless its parent process remains
/// active in the system.
pub fn task_is_stalled(
    last_seen: u64,
    _task_type: Option<TaskType>,
    scheduler_settings: &Settings,
) -> (bool, bool) {
    let min_wait_time = scheduler_settings.time_settings.min_wait_time;
    let max_wait_time = scheduler_settings.time_settings.max_wait_time;
    let now = Utc::now().timestamp() as u64;
    let is_stalled = now - min_wait_time > last_seen;
    let must_be_removed = max_wait_time
        .map(|max| now - max > last_seen)
        .unwrap_or(false);
    (is_stalled, must_be_removed)
}

//#[derive(Debug)]
pub struct Scheduler {
    // Keep a cache of jobs on the system. each job_id has an associated job state
    // indicating the current iteration, and allocated resources and its requirements per resource
    tasks_state: RwLock<HashMap<Pid, TaskState>>,

    // Sorted jobs to be executed.
    jobs_queue: RwLock<VecDeque<Pid>>,

    // the db object to store the current state
    // so in case something goes wrong we can retrive the last
    // state from the db and re-construct our scheduler.
    db: Database,

    devices: RwLock<Resources>,
    settings: Settings,
    system: Mutex<System>,
    pid: Pid,
    shutdown_tracker: RwLock<Instant>,
    shutdown_tx: Option<Sender<()>>,
}

impl Scheduler {
    pub fn new(
        settings: Settings,
        devices: Devices,
        shutdown_tx: Option<Sender<()>>,
        db: Database,
    ) -> Result<Self> {
        // get the system resources
        let mut devices = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task: None,
                    },
                )
            })
            .collect::<HashMap<DeviceId, ResourceState>>();

        // System object to track processes
        let system = Mutex::new(System::new());
        let shutdown_tracker = RwLock::new(Instant::now());
        let pid = palaver::thread::gettid();

        let mut tasks_state = HashMap::new();
        let mut jobs_queue = VecDeque::new();
        //loading jobs from previous session
        for res in db.iter::<Pid, TaskState>() {
            let (key, value) = res??;
            value.allocation.devices.iter().for_each(|id| {
                let _ = devices
                    .get_mut(id)
                    .map(|dev| dev.update_memory_usage(&value.allocation.requirement.resource));
            });
            tasks_state.insert(key, value);
        }

        if !tasks_state.is_empty() {
            let mut solver = create_solver(None);
            jobs_queue = solver.solve_job_schedule(&tasks_state, &settings)?;
        }
        let devices = RwLock::new(Resources(devices));

        Ok(Self {
            tasks_state: RwLock::new(tasks_state),
            jobs_queue: RwLock::new(jobs_queue),
            devices,
            settings,
            db,
            system,
            pid,
            shutdown_tracker,
            shutdown_tx,
        })
    }

    #[instrument(level = "info", skip(requirements, self))]
    pub fn schedule(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
    ) -> Result<Option<ResourceAlloc>> {
        // check for stalled jobs and remove those that no longer exists
        // making room for new clients
        self.log_stalled_jobs();

        if requirements.req.is_empty() {
            error!("Schedule request with empty parameters");
            return Err(Error::ResourceReqEmpty);
        }
        // before continuing we need to check if this client.pid is already in the
        // jobs queue meaning there is a collision that we need to avoid
        // by ignoring the request until the previous job is done.
        if self.tasks_state.read().contains_key(&client.pid) {
            warn!(
                "Ignoring request - A client with the same id: {} is already in the queue ",
                client.pid
            );
            return Ok(None);
        }

        let restrictions =
            match_task_devices(requirements.task_type, &self.settings.tasks_settings);

        let resources = self.devices.read();

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if !resources.has_min_available_memory(&requirements) {
            return Ok(None);
        }

        let mut solver = create_solver(None);
        let alloc = match solver.allocate_task(
            &resources,
            &requirements,
            &restrictions,
            &*self.tasks_state.read(),
        ) {
            Some(res) => res,
            _ => return Ok(None), // Should not happen, we filtered lines before
        };
        drop(resources);

        let time: u64 = Utc::now().timestamp() as u64;

        // prepare the task
        let task_state = TaskState {
            requirements,
            allocation: alloc.clone(),
            last_seen: AtomicU64::new(time),
            aborted: AtomicBool::new(false),
            creation_time: time,
            context: client.context,
        };

        let state_clone = task_state.clone();
        self.tasks_state.write().insert(client.pid, state_clone);
        // Update our plan
        let new_plan = match solver.solve_job_schedule(&self.tasks_state.read(), &self.settings) {
            Ok(plan) => {
                debug!("scheduler job_plan {:?}", plan);
                plan
            }
            Err(e) => {
                error!("Solver error: {}", e.to_string());
                self.tasks_state.write().remove(&client.pid);
                return Err(Error::SolverOther(e.to_string()));
            }
        };

        self.db.insert(client.pid, task_state)?;

        // update our resources state
        let mut resources = self.devices.write();
        alloc.devices.iter().for_each(|id| {
            let _ = resources
                .0
                .get_mut(id)
                .map(|dev| dev.update_memory_usage(&alloc.requirement.resource));
        });
        // update our task state and priority queue
        *self.jobs_queue.write() = new_plan;
        // state is not empty so reset the shutdown tracker
        *self.shutdown_tracker.write() = Instant::now();

        Ok(Some(alloc))
    }

    // this client has to wait if another is currently using the resource it shares
    fn wait_for_busy_resources(&self, client: &ClientToken) -> Result<bool> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        let resources = self.devices.read();
        Ok(resources.has_busy_resources(current_task.allocation.devices.as_slice()))
    }

    // update the last_seen counter
    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    fn update_last_seen(&self, client: &ClientToken) -> Result<()> {
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
    fn check_priority_queue(&self, client: Pid) -> Result<bool> {
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
                        if let Some(next_task) = state.get(id) {
                            current_task.allocation.devices.iter().any(|dev_id| {
                                next_task.allocation.devices.iter().any(|id| dev_id == id)
                            })
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>();
                if !sub_queue.is_empty() {
                    Ok(client == *sub_queue[0])
                } else {
                    Err(Error::UnknownClient)
                }
            }
        } else {
            warn!("Queue empty!");
            Err(Error::UnknownClient)
        }
    }

    // check if client was aborted by the user
    fn abort_client(&self, client: &ClientToken) -> Result<bool> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        Ok(current_task.aborted.load(Ordering::Relaxed))
    }

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    pub fn wait_preemptive(&self, client: ClientToken) -> Result<PreemptionResponse> {
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
                    Some((i.clone(), device.available_memory()))
                } else {
                    None
                }
            })
            .collect::<Vec<(DeviceId, u64)>>();
        SchedulerResponse::ListAllocations(Ok(alloc))
    }

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    pub fn release_preemptive(&self, client: ClientToken) {
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

    #[instrument(level = "trace", skip(self), fields(pid = client.pid))]
    pub fn release(&self, client: ClientToken) {
        self.remove_job(client.pid)
    }

    fn abort(&self, clients: Vec<Pid>) -> Result<()> {
        for client in clients.iter() {
            let state = self.tasks_state.read();
            let current_task = state.get(client).ok_or(Error::UnknownClient)?;
            warn!("aborting client: {} from: {}", client, current_task.context);
            current_task.aborted.store(true, Ordering::Relaxed);
            self.db.insert(*client, current_task.clone())?;
        }
        Ok(())
    }

    fn check_process_exist(&self, pid: Pid) -> bool {
        let mut s = self.system.lock();
        s.refresh_process(pid as _)
    }

    // this function logs stalled jobs that appears to be active in the system however,
    // those that do not correspond to any alive process will be removed.
    fn log_stalled_jobs(&self) {
        for (id, remove) in self.get_stalled_jobs().into_iter() {
            // just in case the maintenance thread is not running this removal happens on-demand,
            // more specifically  if there are stalled jobs and calls to wait_preemptive from
            // clients.
            if !self.check_process_exist(id) || remove {
                self.remove_job(id);
                continue;
            }
            // although the job appears to be in the queue(steps above)
            // it might have returned and called release at this point, so it is better to check here.
            if let Some(task) = self.tasks_state.read().get(&id) {
                warn!("Process {}:{} is stalling!!", id, task.context);
            }
        }
    }

    // this function is experimental and might be removed in later versions of the
    // scheduler.
    fn remove_stalled(&self, clients: Vec<Pid>) -> Result<()> {
        let stalled = self.get_stalled_jobs();
        clients
            .into_iter()
            .filter(|to_remove| stalled.iter().any(|stalled_id| stalled_id.0 == *to_remove))
            .for_each(|to_remove| self.remove_job(to_remove));
        Ok(())
    }

    // returns the id of stalling jobs and indicates if the
    // task should be removed according to the configuration file
    fn get_stalled_jobs(&self) -> Vec<(Pid, bool)> {
        let mut stalled = vec![];
        for (job_id, task) in self.tasks_state.read().iter() {
            let (stalls, remove) = task_is_stalled(
                task.last_seen.load(Ordering::Relaxed),
                task.requirements.task_type,
                &self.settings,
            );
            if stalls {
                stalled.push((*job_id, remove));
            }
        }
        stalled
    }

    fn remove_job(&self, id: Pid) {
        // remove job from our priority queue
        self.jobs_queue.write().retain(|pid| *pid != id);
        // remove job from the state and unset any resources that were in used
        if let Some(current_task) = self.tasks_state.write().remove(&id) {
            let mut devices = self.devices.write();
            devices.unset_busy_resources(&current_task.allocation.devices, id);
            if let ResourceType::Gpu(ref m) = current_task.allocation.requirement.resource {
                devices.free_memory(m, current_task.allocation.devices.as_slice());
            }
            let _ = self.db.remove::<_, TaskState>(id);
        }
        if !self.tasks_state.read().is_empty() {
            *self.shutdown_tracker.write() = Instant::now();
        }
    }

    #[instrument(level = "trace", skip(self))]
    fn monitor(&self) -> std::result::Result<MonitorInfo, String> {
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
                    )
                    .0,
                }
            })
            .collect::<Vec<_>>();
        let resources = resources
            .0
            .iter()
            .map(|(id, state)| GpuResource {
                device_id: id.clone(),
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
            RequestMethod::Schedule(client, req) => {
                SchedulerResponse::Schedule(self.schedule(client, req))
            }
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
            RequestMethod::CheckService => SchedulerResponse::CheckService(self.pid),
        };
        let _ = sender.send(response);
    }

    fn maintenance(&self) -> bool {
        let mut _continue = true;
        // remove jobs that no longer exist in the system.
        let mut to_remove = vec![];
        for id in self.jobs_queue.read().iter() {
            if !self.check_process_exist(*id) {
                warn!("Removing job {}. Parent process does not exist", id);
                to_remove.push(*id);
            }
        }

        for id in to_remove.into_iter() {
            self.remove_job(id);
        }

        if let Some(shutdown_timeout) = self.settings.service.shutdown_timeout {
            if self.shutdown_tracker.read().elapsed().as_secs() > shutdown_timeout {
                let _ = self.shutdown_tx.as_ref().map(|tx| {
                    warn!("Closing service after {}s of inactivity", shutdown_timeout);
                    let _ = tx.try_send(());
                    _continue = false;
                });
            }
        }
        _continue
    }
}
