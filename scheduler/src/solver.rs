use std::collections::{HashMap, VecDeque};

use crate::Error;
use common::{Device, ResourceAlloc, ResourceMemory, ResourceReq, ResourceType, TaskRequirements};

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

    pub fn mem_usage(&self) -> u64 {
        self.mem_usage
    }

    pub fn free_memory(&mut self, mem: &ResourceMemory) {
        match mem {
            ResourceMemory::All => self.mem_usage = 0,
            ResourceMemory::Mem(value) => {
                self.mem_usage -= value;
            }
        }
    }

    pub fn set_as_busy(&mut self) {
        self.is_busy = true;
    }

    pub fn set_as_free(&mut self) {
        self.is_busy = false;
    }

    pub fn is_busy(&self) -> bool {
        self.is_busy
    }
}

#[derive(Clone, Debug)]
pub struct Resources(pub HashMap<u64, ResourceState>);

impl Resources {
    pub fn available_memory(&self, exclusive: bool) -> u64 {
        self.0
            .iter()
            .filter(|(_, dev)| dev.is_exclusive == exclusive)
            .map(|(_, dev)| dev.available_memory())
            .sum()
    }

    pub fn has_min_available_memory(&self, requirements: &TaskRequirements) -> bool {
        for req in &requirements.req {
            let selected_resources = self
                .0
                .iter()
                .filter(|(_, dev)| dev.is_exclusive == requirements.exclusive)
                .filter_map(|(_, device)| {
                    if let ResourceType::Gpu(ref mem) = req.resource {
                        match mem {
                            ResourceMemory::All => {
                                if device.mem_usage == 0 {
                                    Some(1)
                                } else {
                                    None
                                }
                            }
                            ResourceMemory::Mem(value) => {
                                if device.available_memory() >= *value {
                                    Some(1)
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                });
            if selected_resources.count() >= req.quantity {
                return true;
            }
        }
        false
    }

    pub fn free_memory(&mut self, mem: &ResourceMemory, devices: &[u64]) {
        for id in devices {
            let _ = self.0.get_mut(id).map(|dev| dev.free_memory(mem));
        }
    }

    pub fn has_busy_resources(&self, devices: &[u64]) -> bool {
        devices
            .iter()
            .any(|id| self.0.get(id).map(|dev| dev.is_busy()).unwrap_or(false))
    }

    pub fn set_busy_resources(&mut self, devices: &[u64]) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_busy());
        });
    }

    pub fn unset_busy_resources(&mut self, devices: &[u64]) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_free());
        });
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
    ) -> Option<(ResourceAlloc, HashMap<u64, ResourceState>)>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_allocation() {
        let devices = common::list_devices();
        println!("DEVICES: {:?}", devices);
        let state_t1 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: false,
                        is_exclusive: devices
                            .exclusive_gpus()
                            .iter()
                            .any(|&i| i == dev.device_id()),
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t1 = Resources(state_t1);

        let task1 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
                quantity: 1,
                preemptible: false,
            }],
            deadline: None,
            exclusive: false,
            estimations: None,
        };
        assert!(devices_t1.has_min_available_memory(&task1));

        let state_t2 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 3,
                        is_busy: false,
                        is_exclusive: devices
                            .exclusive_gpus()
                            .iter()
                            .any(|&i| i == dev.device_id()),
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();

        //does not fit!
        let task2 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
                quantity: 1,
                preemptible: false,
            }],
            deadline: None,
            exclusive: false,
            estimations: None,
        };

        //should fit!
        let task3 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(1)),
                quantity: 2,
                preemptible: false,
            }],
            deadline: None,
            exclusive: false,
            estimations: None,
        };

        //should not fit as it is exclusive only!
        let task4 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(1)),
                quantity: 2,
                preemptible: false,
            }],
            deadline: None,
            exclusive: true,
            estimations: None,
        };

        let devices_t2 = Resources(state_t2);
        assert!(!devices_t2.has_min_available_memory(&task2));
        assert!(devices_t2.has_min_available_memory(&task3));
        assert!(!devices_t2.has_min_available_memory(&task4));
    }
}
