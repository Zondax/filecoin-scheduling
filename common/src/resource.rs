#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceReq {
    pub resource: String,
    pub quantity: usize,
    pub preemptible: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceAlloc {
    pub resource: ResourceReq,
    pub resource_id: usize,
}
