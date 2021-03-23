use chrono::{offset::Utc, DateTime};
use std::collections::{HashMap, VecDeque};

use common::{
    Device, Error, ResourceAlloc, ResourceMemory, ResourceReq, ResourceType, TaskEstimations,
    TaskRequirements,
};

/// Wrapper that add additional information regarding to the Resource
/// memory and usage.
#[derive(Clone, Debug)]
pub struct ResourceState {
    /// Index that points to the Device.
    pub dev: Device,
    /// Current memory in use
    pub mem_usage: u64,
    /// Mark device as exclusive
    pub is_exclusive: bool,
    /// Using resource?
    pub is_busy: bool,
}

#[derive(Clone, Debug)]
pub struct FrontTask {
    pub last_seen: Option<DateTime<Utc>>,
    pub job_id: Option<u32>,
    pub duration: i64,
}

impl FrontTask {
    pub fn set_last_seen_and_duration(&mut self, job_id: u32, req: &TaskRequirements) {
        self.last_seen = Some(Utc::now());
        self.job_id = Some(job_id);
        if let Some(TaskEstimations {
            time_per_iter: t,
            num_of_iter: _,
            exec_time: _,
        }) = req.estimations
        {
            if cfg!(dummy_devices) {
                self.duration = (t.as_secs() + 3) as i64;
            } else {
                self.duration = (t.as_secs() + 300) as i64;
            }
        } else {
            self.duration = 300_i64;
        }
    }

    pub fn set_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }

    pub fn is_front_job(&self, job_id: u32) -> bool {
        if self.job_id.is_none() {
            false
        } else {
            self.job_id.unwrap() == job_id
        }
    }

    pub fn remove_front_job_from_queue(&self) -> bool {
        if self.last_seen.is_none() {
            false
        } else {
            self.last_seen.unwrap() + chrono::Duration::seconds(self.duration) < Utc::now()
        }
    }

    pub fn remove_front_job(&mut self) {
        self.last_seen = None;
        self.duration = 300_i64;
        self.job_id = None;
    }
}

impl ResourceState {
    pub fn available_memory(&self) -> u64 {
        self.dev.memory() - self.mem_usage
    }

    pub fn update_memory_usage(&mut self, resource_type: &ResourceType) {
        if let ResourceType::Gpu(mem) = resource_type {
            match mem {
                ResourceMemory::All => self.mem_usage = self.dev.memory(),
                ResourceMemory::Mem(value) => self.mem_usage += value,
            }
        }
    }

    pub fn set_as_busy(&mut self) {
        self.is_busy = true;
    }

    pub fn set_as_free(&mut self) {
        self.is_busy = false;
    }
}

#[derive(Clone, Debug)]
pub struct Resources(pub Vec<ResourceState>);

impl Resources {
    pub fn available_memory(&self, exclusive: bool) -> u64 {
        self.0
            .iter()
            .filter(|dev| dev.is_exclusive == exclusive)
            .map(|dev| dev.available_memory())
            .sum()
    }

    pub fn free_memory(&mut self, mem: &ResourceMemory, devices: &[u32]) {
        for dev_id in devices {
            self.0
                .iter_mut()
                .filter(|device| device.dev.bus_id() == *dev_id)
                .for_each(|dev| match mem {
                    ResourceMemory::All => dev.mem_usage = 0,
                    ResourceMemory::Mem(value) => dev.mem_usage -= value,
                });
        }
    }

    pub fn has_busy_resources(&self, devices: &[u32]) -> bool {
        self.0.clone().iter().any(|dev| {
            devices
                .iter()
                .any(|d| d == &dev.dev.bus_id() && dev.is_busy)
        })
    }

    pub fn set_busy_resources(&mut self, devices: &[u32]) {
        for dev_id in devices {
            self.0
                .iter_mut()
                .filter(|device| device.dev.bus_id() == *dev_id)
                .for_each(|dev| dev.set_as_busy());
        }
    }

    pub fn unset_busy_resources(&mut self, devices: &[u32]) {
        for dev_id in devices {
            self.0
                .iter_mut()
                .filter(|device| device.dev.bus_id() == *dev_id)
                .for_each(|dev| dev.set_as_free());
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskState {
    pub requirements: TaskRequirements,
    pub current_iteration: u16,
    // The list of jobs associates with this task, each job is a requirement plus the resource
    // assigned to it accordingly.
    pub allocation: ResourceAlloc,
    pub is_stalled: bool,
}

impl TaskState {
    pub fn end_timestamp(&self) -> i64 {
        self.requirements
            .deadline
            .map_or(i64::MAX, |d| d.end_timestamp_secs())
    }
}

#[derive(Clone, Debug)]
pub struct Job {
    pub starting_time: usize,
    pub end_time: usize,
    pub req: ResourceReq,
    pub resources: Vec<u32>,
}

// Trait that is implemented by any object that can be used as a solver
pub trait Solver {
    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
    ) -> Result<VecDeque<u32>, Error>;

    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
    ) -> Option<(ResourceAlloc, Vec<ResourceState>)>;
}
