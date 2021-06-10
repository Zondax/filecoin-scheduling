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

//this is more appropriate here, unless this is VERY specific
impl TaskType {
    pub fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let mut s = String::deserialize(de)?;
        s.make_ascii_lowercase();

        match s.as_ref() {
            "merkleproof" => Ok(TaskType::MerkleProof),
            "winningpost" => Ok(TaskType::WinningPost),
            "windowpost" => Ok(TaskType::WindowPost),
            _ => Err(serde::de::Error::custom(
                "error trying to deserialize rotation policy config",
            )),
        }
    }
}

impl TaskResult {
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }
}

/// Deadline struct to configure when the task should be started and finished
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Deadline {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Deadline {
    pub fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> Self {
        Self { start, end: finish }
    }

    pub fn from_secs(start: u64, end: u64) -> Self {
        let start = chrono::Utc::now() + chrono::Duration::seconds(start as _);
        let end = start + chrono::Duration::seconds(end as _);
        Self::new(start, end)
    }

    pub fn default_now() -> Self {
        let start = chrono::Utc::now();
        Self::new(start, start)
    }

    pub fn start_timestamp_secs(&self) -> i64 {
        self.start.timestamp()
    }

    pub fn end_timestamp_secs(&self) -> i64 {
        self.end.timestamp()
    }
}

/// Contains all the timing descriptions for
/// a task. These parameters will be used by the scheduler solve for
/// scheduling the task in the right time window and resource
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskEstimations {
    pub time_per_iter: Duration,
    pub num_of_iter: usize,
}

#[derive(Default)]
pub struct TaskReqBuilder {
    req: Vec<ResourceReq>,
    deadline: Option<Deadline>,
    task_estimations: Option<TaskEstimations>,
    task_type: Option<TaskType>,
}

impl TaskReqBuilder {
    pub fn new() -> Self {
        Self {
            req: vec![],
            ..Default::default()
        }
    }

    pub fn resource_req(mut self, req: ResourceReq) -> Self {
        self.req.push(req);
        self
    }

    pub fn with_deadline(mut self, deadline: Option<Deadline>) -> Self {
        self.deadline = deadline;
        self
    }

    pub fn with_time_estimations(mut self, time_per_iter: Duration, num_of_iter: usize) -> Self {
        self.task_estimations.replace(TaskEstimations {
            time_per_iter,
            num_of_iter,
        });
        self
    }

    pub fn with_task_type(mut self, task: TaskType) -> Self {
        self.task_type = Some(task);
        self
    }

    pub fn build(self) -> TaskRequirements {
        TaskRequirements {
            req: self.req,
            deadline: self.deadline,
            estimations: self.task_estimations,
            task_type: self.task_type,
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
    pub estimations: Option<TaskEstimations>,
    pub task_type: Option<TaskType>,
}

impl TaskRequirements {
    /// Returns the minimal amount of memory required
    pub fn minimal_resource_usage(&self) -> u64 {
        self.req
            .iter()
            .filter_map(|req| {
                if let ResourceType::Gpu(ResourceMemory::Mem(value)) = req.resource {
                    Some(value * (req.quantity as u64))
                } else {
                    None
                }
            })
            //skip an allocation
            // iterate over everything only once
            // sort right away with at most N comparisons
            // since you discard items > current
            // if the iterator is empty this would do nothing
            .min()
            .unwrap_or_default()
    }
}
