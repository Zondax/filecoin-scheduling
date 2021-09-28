use crate::DeviceId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Device, Pid};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Cpu,
    // Use a Gpu and Define how much memory we want.
    Gpu(ResourceMemory),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceMemory {
    // Wants to use all the resource's memory
    All,
    // Indicates the amount of memory to use
    Mem(u64),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReq {
    pub resource: ResourceType,
    // quantity of resources of this type needed
    pub quantity: usize,
    pub preemptible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAlloc {
    pub requirement: ResourceReq,
    // the devices allowed to use
    pub devices: Vec<DeviceId>,
}

impl Default for ResourceAlloc {
    fn default() -> Self {
        Self {
            requirement: ResourceReq {
                resource: ResourceType::Cpu,
                quantity: 0,
                preemptible: false,
            },
            devices: vec![],
        }
    }
}

/// Wrapper that add additional information regarding to the Resource
/// memory and usage.
#[derive(Clone, Debug)]
pub struct ResourceState {
    /// Index that points to the Device.
    pub dev: Device,
    /// Current memory in use
    pub mem_usage: u64,
    /// The task that is using this resource
    /// None means the resource is free
    pub current_task: Option<Pid>,
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

    pub fn set_as_busy(&mut self, task: Pid) {
        // It is an error trying to set as busy a resource that is being used by
        // another process. It means that the scheduler is allowing multiple task
        // to use a resource at the same time.
        debug_assert!(
            self.current_task.is_none(),
            "Resource already in used -> multiple process trying to use it at the same time"
        );
        self.current_task.replace(task);
    }

    pub fn set_as_free(&mut self, task: Pid) {
        // only the task that set the resource as busy can freed it
        if Some(task) == self.current_task {
            self.current_task.take();
        }
    }

    pub fn current_task(&self) -> Option<Pid> {
        self.current_task
    }

    pub fn is_busy(&self) -> bool {
        self.current_task.is_some()
    }
}

#[derive(Clone, Debug)]
pub struct Resources(pub HashMap<DeviceId, ResourceState>);

impl Resources {
    pub fn available_memory(&self) -> u64 {
        self.0.iter().map(|(_, dev)| dev.available_memory()).sum()
    }

    pub fn get_devices_with_requirements<'r>(
        &'r self,
        requirements: &'r ResourceReq,
    ) -> impl Iterator<Item = DeviceId> + 'r {
        self.0
            .iter()
            .filter_map(move |(sel, dev)| {
                if let ResourceType::Gpu(mem) = &requirements.resource {
                    match mem {
                        //The caller will handle all remaining memory
                        ResourceMemory::All => Some(sel),
                        ResourceMemory::Mem(val) => {
                            if dev.available_memory() >= *val {
                                Some(sel)
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            })
            .cloned()
    }

    ///Indicates if these resources can accomodate at least 1 of the resource requests
    /// for the given task
    pub fn has_min_available_memory(&self, requirements: &[ResourceReq]) -> bool {
        for req in requirements {
            let n_res_with_memory = self.get_devices_with_requirements(req).count();

            if n_res_with_memory >= req.quantity {
                return true;
            }
        }
        false
    }

    pub fn free_memory(&mut self, mem: &ResourceMemory, devices: &[DeviceId]) {
        for id in devices {
            let _ = self.0.get_mut(id).map(|dev| dev.free_memory(mem));
        }
    }

    pub fn has_busy_resources(&self, devices: &[DeviceId]) -> bool {
        devices
            .iter()
            .any(|id| self.0.get(id).map(|dev| dev.is_busy()).unwrap_or(false))
    }

    pub fn set_busy_resources(&mut self, devices: &[DeviceId], task: Pid) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_busy(task));
        });
    }

    pub fn unset_busy_resources(&mut self, devices: &[DeviceId], task: Pid) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_free(task));
        });
    }
}
