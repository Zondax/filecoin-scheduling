use std::collections::{HashMap, VecDeque};

use serde::{de::Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use crate::config::Settings;
use crate::Error;
use common::{
    Device, ResourceAlloc, ResourceMemory, ResourceReq, ResourceType, TaskId, TaskRequirements,
};
use rust_gpu_tools::opencl::GPUSelector;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

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
    pub current_task: Option<TaskId>,
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

    pub fn set_as_busy(&mut self, task: TaskId) {
        // It is an error trying to set as busy a resource that is being used by
        // another process. It means that the scheduler is allowing multiple task
        // to use a resource at the same time.
        debug_assert!(
            self.current_task.is_none(),
            "Resource already in used -> multiple process trying to use it at the same time"
        );
        self.current_task.replace(task);
    }

    pub fn set_as_free(&mut self, task: TaskId) {
        // only the task that set the resource as busy can freed it
        if Some(task) == self.current_task {
            self.current_task.take();
        }
    }

    pub fn current_task(&self) -> Option<TaskId> {
        self.current_task
    }

    pub fn is_busy(&self) -> bool {
        self.current_task.is_some()
    }
}

#[derive(Clone, Debug)]
pub struct Resources(pub HashMap<GPUSelector, ResourceState>);

impl Resources {
    pub fn available_memory(&self) -> u64 {
        self.0.iter().map(|(_, dev)| dev.available_memory()).sum()
    }

    pub fn get_devices_with_requirements<'r>(
        &'r self,
        requirements: &'r ResourceReq,
    ) -> impl Iterator<Item = GPUSelector> + 'r {
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
    pub fn has_min_available_memory(&self, requirements: &TaskRequirements) -> bool {
        for req in &requirements.req {
            let n_res_with_memory = self.get_devices_with_requirements(req).count();

            if n_res_with_memory >= req.quantity {
                return true;
            }
        }
        false
    }

    pub fn free_memory(&mut self, mem: &ResourceMemory, devices: &[GPUSelector]) {
        for id in devices {
            let _ = self.0.get_mut(id).map(|dev| dev.free_memory(mem));
        }
    }

    pub fn has_busy_resources(&self, devices: &[GPUSelector]) -> bool {
        devices
            .iter()
            .any(|id| self.0.get(id).map(|dev| dev.is_busy()).unwrap_or(false))
    }

    pub fn set_busy_resources(&mut self, devices: &[GPUSelector], task: TaskId) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_busy(task));
        });
    }

    pub fn unset_busy_resources(&mut self, devices: &[GPUSelector], task: TaskId) {
        devices.iter().for_each(|id| {
            let _ = self.0.get_mut(id).map(|dev| dev.set_as_free(task));
        });
    }
}

fn serialize_atomic_u64<S>(v: &AtomicU64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(v.load(Ordering::Relaxed))
}

fn deserialize_atomic_u64<'de, D>(de: D) -> Result<AtomicU64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;

    match s.parse::<u64>() {
        Ok(value) => Ok(AtomicU64::new(value)),
        Err(_) => Err(serde::de::Error::custom(
            "error trying to deserialize u64 for task last_seen timestamp",
        )),
    }
}

fn serialize_atomic_bool<S>(v: &AtomicBool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_bool(v.load(Ordering::Relaxed))
}

fn deserialize_atomic_bool<'de, D>(de: D) -> Result<AtomicBool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;

    match s.parse::<bool>() {
        Ok(value) => Ok(AtomicBool::new(value)),
        Err(_) => Err(serde::de::Error::custom(
            "error trying to deserialize boolean for task abort flag",
        )),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskState {
    pub requirements: TaskRequirements,
    // the list of resources this task is using
    pub allocation: ResourceAlloc,

    #[serde(
        deserialize_with = "deserialize_atomic_u64",
        serialize_with = "serialize_atomic_u64"
    )]
    pub last_seen: AtomicU64,

    #[serde(
        deserialize_with = "deserialize_atomic_bool",
        serialize_with = "serialize_atomic_bool"
    )]
    pub aborted: AtomicBool,
    // a timestamp indicating when this task was created
    pub creation_time: u64,
    pub context: Option<String>,
}

impl Clone for TaskState {
    fn clone(&self) -> Self {
        Self {
            requirements: self.requirements.clone(),
            allocation: self.allocation.clone(),
            last_seen: AtomicU64::new(self.last_seen.load(Ordering::Relaxed)),
            aborted: AtomicBool::new(self.aborted.load(Ordering::Relaxed)),
            creation_time: self.creation_time,
            context: self.context.clone(),
        }
    }
}

impl TaskState {
    pub fn end_timestamp(&self) -> i64 {
        self.requirements
            .deadline
            .map_or(i64::MAX, |d| d.end_timestamp_secs())
    }
}

// Trait that is implemented by any object that can be used as a solver
pub trait Solver {
    fn solve_job_schedule(
        &mut self,
        input: &HashMap<TaskId, TaskState>,
        scheduler_settings: &Settings,
    ) -> Result<VecDeque<TaskId>, Error>;

    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
        restrictions: &Option<Vec<GPUSelector>>,
    ) -> Option<(ResourceAlloc, HashMap<GPUSelector, ResourceState>)>;
}

#[cfg(dummy_devices)]
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
                        current_task: None,
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
            estimations: None,
            task_type: None,
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
                        current_task: None,
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
            estimations: None,
            task_type: None,
        };

        //should fit!
        let task3 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(1)),
                quantity: 2,
                preemptible: false,
            }],
            deadline: None,
            estimations: None,
            task_type: None,
        };

        let devices_t2 = Resources(state_t2);
        assert!(!devices_t2.has_min_available_memory(&task2));
        assert!(devices_t2.has_min_available_memory(&task3));
    }
}
