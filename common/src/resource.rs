#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
    Gpu,
    Cpu,
    GpuMemory(u64),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceReq {
    pub resource: ResourceType,
    pub quantity: usize,
    pub preemptible: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceAlloc {
    pub resource: ResourceReq,
    pub resource_id: u32,
}
