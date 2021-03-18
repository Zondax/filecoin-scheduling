use chrono::{offset::Utc, DateTime};
use std::error::Error;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{ResourceAlloc, ResourceMemory, ResourceReq, ResourceType};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait TaskFunc {
    type TaskOutput;

    fn init(&mut self, _: &ResourceAlloc) -> Result<()> {
        Ok(())
    }
    fn end(&mut self, _: &ResourceAlloc) -> Result<Self::TaskOutput>;
    fn task(&mut self, alloc: &ResourceAlloc) -> TaskResult;
}

/// Helper type that indicates if a task should be executed again
#[derive(PartialEq, Eq)]
pub enum TaskResult {
    Continue,
    Done,
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

/// Contains the functions for initializacion, finalization and main task function
/// along with the requirements for this task to be executed and scheduled
pub struct Task<T> {
    pub task_func: Box<dyn TaskFunc<TaskOutput = T>>,
    pub task_req: TaskRequirements,
}

impl<T> Task<T> {
    pub fn new(func: impl TaskFunc<TaskOutput = T> + 'static, task_req: TaskRequirements) -> Self {
        Self {
            task_func: Box::new(func),
            task_req,
        }
    }

    pub fn default(func: impl TaskFunc<TaskOutput = T> + 'static) -> Self {
        let req = vec![ResourceReq {
            resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        }];
        let time_per_iter = Duration::from_millis(500);
        let exec_time = Duration::from_millis(3000);
        let start = Utc::now();
        let end = start + chrono::Duration::seconds(30);
        let deadline = Some(Deadline::new(start, end));
        let num_of_iter = 1;

        let task_requirements = TaskRequirements {
            req,
            deadline,
            exclusive: false,
            estimations: Some(TaskEstimations {
                time_per_iter,
                num_of_iter,
                exec_time,
            }),
        };
        Self::new(func, task_requirements)
    }
}
