use chrono::{offset::Utc, DateTime};
use std::error::Error;
use std::time::Duration;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{ResourceAlloc, ResourceReq};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Helper time to describe the different returns types of a task
pub enum TaskResult<T> {
    Continue,
    Done(Result<T>),
}

/// Deadline struct to configure when the task should be started and finished
#[derive(Debug, Serialize, Deserialize)]
pub struct Deadline(DateTime<Utc>, DateTime<Utc>);

impl Deadline {
    pub fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> Self {
        Self(start, finish)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub(crate) req: ResourceReq,
    pub(crate) time_per_iter: Duration,
    pub(crate) exec_time: Duration,
    pub(crate) deadline: Deadline,
}

pub struct Task<T> {
    //#[serde(skip_serializing)]
    pub task: Box<dyn Fn(Vec<ResourceAlloc>) -> TaskResult<T>>,
    pub task_req: TaskRequirements,
}

impl<T: Serialize + DeserializeOwned> Task<T> {
    pub fn new(
        func: impl Fn(Vec<ResourceAlloc>) -> TaskResult<T> + 'static,
        req: ResourceReq,
        time_per_iter: Duration,
        exec_time: Duration,
        deadline: Deadline,
    ) -> Self {
        let task_req = TaskRequirements {
            req,
            time_per_iter,
            exec_time,
            deadline,
        };
        Self {
            task: Box::new(func),
            task_req,
        }
    }
}
