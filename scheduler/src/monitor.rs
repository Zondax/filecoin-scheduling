use serde::{Deserialize, Serialize};

use common::{Deadline, ResourceAlloc, TaskType};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub task_states: Vec<Task>,
    pub resources: Vec<GpuResource>,
    pub job_plan: Vec<u32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub alloc: ResourceAlloc,
    pub task_type: Option<TaskType>,
    pub deadline: Option<Deadline>,
    pub last_seen: u64,
    pub stalled: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GpuResource {
    pub name: String,
    pub device_id: u64,
    pub memory: u64,
    pub mem_usage: u64,
    pub is_busy: bool,
}
