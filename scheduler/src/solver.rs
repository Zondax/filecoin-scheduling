use std::collections::{HashMap, VecDeque};

use common::{
    Device, Error, ResourceAlloc, ResourceMemory, ResourceReq, ResourceType, TaskRequirements,
};

/// Wrapper that add additional information regarding to the Resource
/// memory and usage.
#[derive(Clone, Debug)]
pub struct ResourceState {
    /// Index that points to the Device.
    pub dev: Device,
    /// Current memory in use
    pub mem_usage: u64,
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
}

#[derive(Clone, Debug)]
pub struct Resources(pub Vec<ResourceState>);

impl Resources {
    pub fn available_memory(&self) -> u64 {
        self.0.iter().map(|dev| dev.available_memory()).sum()
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
}

#[derive(Clone, Debug)]
pub struct TaskState {
    pub requirements: TaskRequirements,
    pub current_iteration: u16,
    // The list of jobs associates with this task, each job is a requirement plus the resource
    // assigned to it accordingly.
    pub allocation: ResourceAlloc,
}

impl TaskState {
    pub fn end_timestamp(&self) -> i64 {
        self.requirements.deadline.end_timestamp_secs()
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
