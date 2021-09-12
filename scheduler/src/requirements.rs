use std::time::Duration;

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

use super::ResourceReq;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    MerkleTree,
    WinningPost,
    WindowPost,
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

    pub fn as_duration(&self) -> Option<Duration> {
        let start = self.start_timestamp_secs();
        let end = self.end_timestamp_secs();
        end.checked_sub(start)
            .map(|duration_secs| Duration::from_secs(duration_secs as u64))
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

    // TODO: evaluate if this method should return an error
    // in case the resource_req list is empty
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

// Creates a dummy task requirements that is useful for testing purposes
pub fn dummy_task_requirements() -> TaskRequirements {
    use super::{ResourceMemory, ResourceType};

    let start = chrono::Utc::now();
    let end = start + chrono::Duration::seconds(30);
    let deadline = Deadline::new(start, end);

    TaskReqBuilder::new()
        .resource_req(ResourceReq {
            resource: ResourceType::Gpu(ResourceMemory::All),
            quantity: 1,
            preemptible: true,
        })
        .with_time_estimations(Duration::from_millis(500), 1)
        .with_deadline(Some(deadline))
        .build()
}
