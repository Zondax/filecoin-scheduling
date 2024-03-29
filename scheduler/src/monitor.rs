use serde::{Deserialize, Serialize};

use crate::{Deadline, DeviceId, Pid, ResourceAlloc, TaskType};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct MonitorInfo {
    pub task_states: Vec<Task>,
    pub resources: Vec<GpuResource>,
    pub job_plan: Vec<Pid>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: Pid,
    pub alloc: ResourceAlloc,
    pub task_type: Option<TaskType>,
    pub deadline: Option<Deadline>,
    pub last_seen: u64,
    pub stalled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GpuResource {
    pub name: String,
    pub device_id: DeviceId,
    pub memory: u64,
    pub mem_usage: u64,
    pub is_busy: bool,
}
