use chrono::{offset::Utc, DateTime};
use std::error::Error;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};

use super::{ResourceAlloc, ResourceReq};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Helper time to describe the different returns types of a task
pub enum TaskResult<T> {
    Continue,
    Done(Result<T>),
}

/// Deadline struct to configure when the task should be started and finished
pub struct Deadline(DateTime<Utc>, DateTime<Utc>);

impl Deadline {
    pub fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> Self {
        Self(start, finish)
    }
}

pub struct Task<T> {
    pub task: Box<dyn Fn(Vec<ResourceAlloc>) -> TaskResult<T>>,
    pub(crate) req: ResourceReq,
    pub(crate) time_per_iter: Duration,
    pub(crate) exec_time: Duration,
    pub(crate) deadline: Deadline,
}

impl<T: Serialize + DeserializeOwned> Task<T> {
    pub fn new(
        func: impl Fn(Vec<ResourceAlloc>) -> TaskResult<T> + 'static,
        req: ResourceReq,
        time_per_iter: Duration,
        exec_time: Duration,
        deadline: Deadline,
    ) -> Self {
        Self {
            task: Box::new(func),
            req,
            time_per_iter,
            exec_time,
            deadline,
        }
    }
}
