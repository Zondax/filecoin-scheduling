use std::collections::HashMap;
use std::collections::VecDeque;
use sysinfo::{System, SystemExt};

use parking_lot::{Mutex, RwLock};
use std::time::Instant;
use tracing::{debug, error, instrument, warn};

use crate::config::Settings;
use crate::db::Database;
use crate::monitor::{GpuResource, MonitorInfo, Task as MonitorTask};
use crate::requests::SchedulerResponse;
use crate::solvers::create_solver;

use crate::{
    ClientToken, DeviceId, Devices, Pid, PreemptionResponse, ResourceAlloc, ResourceType,
    TaskRequirements,
};
use crate::{Error, Result};
use crate::{ResourceState, Resources, TaskState};

//#[derive(Debug)]
pub struct Scheduler {
    // Keep a cache of jobs on the system. each job_id has an associated job state
    // indicating the current iteration, and allocated resources and its requirements per resource
    pub(crate) tasks_state: RwLock<HashMap<Pid, TaskState>>,

    // Sorted jobs to be executed.
    pub(crate) jobs_queue: RwLock<VecDeque<Pid>>,

    // the db object to store the current state
    // so in case something goes wrong we can retrive the last
    // state from the db and re-construct our scheduler.
    pub(crate) db: Database,

    pub(crate) devices: RwLock<Resources>,
    pub(crate) settings: Settings,
    pub(crate) system: Mutex<System>,
    pub(crate) pid: Pid,
    pub(crate) shutdown_tracker: RwLock<Instant>,
}

impl Scheduler {
    pub fn new(settings: Settings, devices: Devices, db: Database) -> Result<Self> {
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
            jobs_queue = solver.make_plan(&tasks_state, &settings)?;
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

        let state = self.tasks_state.read();
        // before continuing we need to check if this client.pid is already in the
        // jobs queue meaning there is a collision that we need to avoid
        // by ignoring the request until the previous job is done.
        if state.contains_key(&client.pid) {
            warn!(
                "Ignoring request - A client with the same id: {} is already in the queue ",
                client.pid
            );
            return Ok(None);
        }

        let restrictions = self.settings.devices_for_task(requirements.task_type);

        let resources = self.devices.read();

        // First step is to check if there are enough resources. This avoids calling alloc
        // knowing that it might fail
        if !resources.has_min_available_memory(&requirements.req) {
            return Ok(None);
        }

        let mut solver = create_solver(None);
        let alloc = match solver.allocate_task(&resources, &requirements, restrictions, &state) {
            Some(res) => res,
            _ => return Ok(None), // Should not happen, we filtered lines before
        };
        drop(resources);
        drop(state);

        let task_state = TaskState::new(requirements, alloc.clone(), client.context);

        let state_clone = task_state.clone();
        self.tasks_state.write().insert(client.pid, state_clone);

        // Update our plan
        let res = { solver.make_plan(&self.tasks_state.read(), &self.settings) };

        let new_plan = match res {
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

        if let Err(e) = self.db.insert(client.pid, task_state) {
            warn!("Can not add task to internal Database {}", e.to_string());
        }

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
        self.tasks_state
            .read()
            .get(&client.pid)
            .ok_or(Error::UnknownClient)?
            .update_last_seen();
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
        let state = self.tasks_state.read();

        if queue.is_empty() {
            return Err(Error::UnknownClient);
        }
        debug!("current job_plan {:?}", *queue);

        if queue.front().filter(|v| **v == client).is_some() {
            return Ok(true);
        }

        // Slow path where the client is not at the front so we need to get a sub-queue
        // that contains all the tasks(including client) that share at least one resource
        // this sub-queue is ordered according to each task's priority
        let current_task = state.get(&client).ok_or(Error::UnknownClient)?;
        let dont_wait = queue
            .iter()
            .filter(|id| {
                if let Some(next_task) = state.get(id) {
                    current_task
                        .allocation
                        .devices
                        .iter()
                        .any(|dev_id| next_task.allocation.devices.contains(dev_id))
                } else {
                    false
                }
            })
            // we care about the first element as it is the highest
            // priority task that uses at least one resource as client
            // then compare it to check if client can continue
            // immediately
            .take(1)
            .map(|task| *task == client)
            .next();
        dont_wait.ok_or(Error::UnknownClient)
    }

    // check if a task was cancelled by the user
    fn abort_client(&self, client: &ClientToken) -> Result<bool> {
        let state = self.tasks_state.read();
        let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
        Ok(current_task.aborted())
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
    pub fn list_allocations(&self) -> SchedulerResponse {
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

    pub fn abort(&self, clients: Vec<Pid>) -> Result<()> {
        for client in clients.iter() {
            let state = self.tasks_state.read();
            let current_task = state.get(client).ok_or(Error::UnknownClient)?;
            warn!("aborting client: {} from: {}", client, current_task.context);
            current_task.abort();
            self.db.insert(*client, current_task.clone())?;
        }
        Ok(())
    }

    pub fn check_process_exist(&self, pid: Pid) -> bool {
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
    pub fn remove_stalled(&self, clients: Vec<Pid>) -> Result<()> {
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
            let (stalls, remove) = task.is_stalling(&self.settings);
            if stalls {
                stalled.push((*job_id, remove));
            }
        }
        stalled
    }

    pub fn remove_job(&self, id: Pid) {
        //Get writers before removing the task from the queue and the state
        //as this avoids race conditions
        let mut queue = self.jobs_queue.write();
        let mut state = self.tasks_state.write();
        // remove job from our priority queue
        queue.retain(|pid| *pid != id);
        let task = state.remove(&id);
        let is_empty = state.is_empty();
        drop(queue);
        drop(state);

        // remove job from the state and unset any resources that were in used
        if let Some(current_task) = task {
            let mut devices = self.devices.write();
            devices.unset_busy_resources(&current_task.allocation.devices, id);
            if let ResourceType::Gpu(ref m) = current_task.allocation.requirement.resource {
                devices.free_memory(m, current_task.allocation.devices.as_slice());
            }
            let _ = self.db.remove::<_, TaskState>(id);
        }

        if !is_empty {
            *self.shutdown_tracker.write() = Instant::now();
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn monitor(&self) -> std::result::Result<MonitorInfo, String> {
        let task_states = self.tasks_state.read();
        let resources = self.devices.read();
        let queue = self.jobs_queue.read();
        let task_states = task_states
            .iter()
            .map(|(id, task)| MonitorTask {
                id: *id,
                alloc: task.allocation.clone(),
                task_type: task.requirements.task_type,
                deadline: task.requirements.deadline,
                last_seen: task.last_seen(),
                stalled: task.is_stalling(&self.settings).0,
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
            job_plan: queue.iter().copied().collect::<Vec<_>>(),
        })
    }
}
