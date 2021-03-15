use chrono::{offset::Utc, DateTime};
use std::error::Error;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{ResourceAlloc, ResourceMemory, ResourceReq, ResourceType};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub type InitFuncType = Option<Box<dyn Fn(&ResourceAlloc) -> Result<()>>>;
pub type EndFuncType = Option<Box<dyn Fn(&ResourceAlloc) -> Result<()>>>;

/// Helper type to describe the different returns types of a task
pub enum TaskResult<T> {
    Continue,
    Done(Result<T>),
}

impl<T> TaskResult<T> {
    pub fn get_result(self) -> Option<Result<T>> {
        match self {
            Self::Continue => None,
            Self::Done(res) => Some(res),
        }
    }

    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }
}

/// Deadline struct to configure when the task should be started and finished
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub exec_time: Duration,
    pub deadline: Deadline,
}

/// Contains all the requirements and timing description for
/// a task. This parameter will be used by the scheduler solve for
/// scheduling the task in the right time window and resource
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub req: Vec<ResourceReq>,
    pub estimations: TaskEstimations,
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
    //#[serde(skip_serializing)]
    pub init: InitFuncType,
    pub end: EndFuncType,
    pub task: Box<dyn FnMut(&ResourceAlloc) -> TaskResult<T>>,
    pub task_req: TaskRequirements,
}

impl<T> Task<T> {
    // TODO:
    // There is an error if we do not use the Option<Box<dyn Fn>>, type for init and end functions.
    // It makes lesser handy the construction of this type. may be we can try a TaskBuilder
    // approach
    pub fn new(
        func: impl FnMut(&ResourceAlloc) -> TaskResult<T> + 'static,
        init: InitFuncType,
        end: EndFuncType,
        task_req: TaskRequirements,
    ) -> Self {
        Self {
            task: Box::new(func),
            init,
            end,
            task_req,
        }
    }

    pub fn default(func: impl FnMut(&ResourceAlloc) -> TaskResult<T> + 'static) -> Self {
        let req = vec![ResourceReq {
            resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        }];
        let time_per_iter = Duration::from_millis(500);
        let exec_time = Duration::from_millis(3000);
        let start = Utc::now();
        let end = start + chrono::Duration::seconds(3);
        let deadline = Deadline::new(start, end);

        let task_requirements = TaskRequirements {
            req,
            estimations: TaskEstimations {
                time_per_iter,
                exec_time,
                deadline,
            },
        };
        Self::new(func, None, None, task_requirements)
    }
}
