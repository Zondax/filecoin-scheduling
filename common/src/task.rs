use chrono::{offset::Utc, DateTime};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{ResourceAlloc, ResourceMemory, ResourceReq, ResourceType};

pub trait TaskFunc {
    type Output;
    type Error;

    fn init(&mut self, _: Option<&ResourceAlloc>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error>;
    fn task(&mut self, alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error>;
}

/// Helper type that indicates if a task should be executed again
#[derive(PartialEq, Eq)]
pub enum TaskResult {
    Continue,
    Done,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum TaskType {
    MerkleProof,
    WinningPost,
    WindowPost,
}

impl TaskResult {
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }
}

/// Deadline struct to configure when the task should be started and finished
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Deadline(pub DateTime<Utc>, pub DateTime<Utc>);

impl Deadline {
    pub fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> Self {
        Self(start, finish)
    }

    pub fn start_timestamp_secs(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn end_timestamp_secs(&self) -> i64 {
        self.1.timestamp()
    }
}

/// Contains all the timing descriptions for
/// a task. These parameters will be used by the scheduler solve for
/// scheduling the task in the right time window and resource
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskEstimations {
    pub time_per_iter: Duration,
    pub num_of_iter: usize,
    pub exec_time: Duration,
}

#[derive(Default)]
pub struct TaskReqBuilder {
    req: Vec<ResourceReq>,
    exclusive: bool,
    deadline: Option<Deadline>,
    task_estimations: Option<TaskEstimations>,
}

impl TaskReqBuilder {
    pub fn new() -> Self {
        Self {
            req: vec![],
            exclusive: false,
            ..Default::default()
        }
    }

    pub fn resource_req(mut self, req: ResourceReq) -> Self {
        self.req.push(req);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
        self
    }

    pub fn with_deadline(mut self, deadline: Option<Deadline>) -> Self {
        self.deadline = deadline;
        self
    }

    pub fn with_time_estimations(
        mut self,
        time_per_iter: Duration,
        num_of_iter: usize,
        exec_time: Duration,
    ) -> Self {
        self.task_estimations.replace(TaskEstimations {
            time_per_iter,
            num_of_iter,
            exec_time,
        });
        self
    }

    pub fn build(self) -> TaskRequirements {
        TaskRequirements {
            req: self.req,
            deadline: self.deadline,
            exclusive: self.exclusive,
            estimations: self.task_estimations,
        }
    }
}

/// Contains all the requirements and timing description for
/// a task. This parameter will be used by the scheduler solve for
/// scheduling the task in the right time window and resource
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub req: Vec<ResourceReq>,
    pub deadline: Option<Deadline>,
    pub exclusive: bool,
    pub estimations: Option<TaskEstimations>,
}

impl TaskRequirements {
    /// Returns the minimal amount of memory required
    pub fn minimal_resource_usage(&self) -> u64 {
        let mut mem_resource_list = self
            .req
            .iter()
            .filter_map(|req| {
                if let ResourceType::Gpu(ResourceMemory::Mem(value)) = req.resource {
                    return Some(value * (req.quantity as u64));
                }
                None
            })
            .collect::<Vec<u64>>();
        if !mem_resource_list.is_empty() {
            mem_resource_list.sort_unstable();
            mem_resource_list[0]
        } else {
            0
        }
    }
}
