use rust_gpu_tools::opencl::GPUSelector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Cpu,
    // Use a Gpu and Define how much memory we want.
    Gpu(ResourceMemory),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceMemory {
    // Wants to use all the resource's memory
    All,
    // Indicates the amount of memory to use
    Mem(u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReq {
    pub resource: ResourceType,
    // quantity of resources of this type needed
    pub quantity: usize,
    pub preemptible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAlloc {
    pub requirement: ResourceReq,
    // the devices allowed to use
    pub devices: Vec<GPUSelector>,
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
