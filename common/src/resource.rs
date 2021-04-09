#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
    Cpu,
    // Use a Gpu and Define how much memory we want.
    Gpu(ResourceMemory),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ResourceMemory {
    // Wants to use all the resource's memory
    All,
    // Indicates the amount of memory to use
    Mem(u64),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResourceReq {
    pub resource: ResourceType,
    pub quantity: usize,
    pub preemptible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResourceAlloc {
    pub requirement: ResourceReq,
    pub resource_id: Vec<u64>,
}

impl Default for ResourceAlloc {
    fn default() -> Self {
        Self {
            requirement: ResourceReq {
                resource: ResourceType::Cpu,
                quantity: 0,
                preemptible: false,
            },
            resource_id: vec![],
        }
    }
}
