use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceReq {
    resource: String,
    quantity: usize,
    preemptible: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceAlloc {
    resource: ResourceReq,
    resource_id: usize,
}
