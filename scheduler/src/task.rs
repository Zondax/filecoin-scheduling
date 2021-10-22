use chrono::Utc;
use serde::{de::Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use crate::{ResourceAlloc, Settings, TaskRequirements};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Process id
pub type Pid = u64;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    MerkleTree,
    WinningPost,
    WindowPost,
}

fn serialize_atomic_u64<S>(v: &AtomicU64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(v.load(Ordering::Relaxed))
}

fn deserialize_atomic_u64<'de, D>(de: D) -> Result<AtomicU64, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(de) {
        Ok(value) => Ok(AtomicU64::new(value)),
        Err(_) => Err(serde::de::Error::custom(
            "error trying to deserialize u64 for task last_seen timestamp",
        )),
    }
}

fn serialize_atomic_bool<S>(v: &AtomicBool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_bool(v.load(Ordering::Relaxed))
}

fn deserialize_atomic_bool<'de, D>(de: D) -> Result<AtomicBool, D::Error>
where
    D: Deserializer<'de>,
{
    match bool::deserialize(de) {
        Ok(value) => Ok(AtomicBool::new(value)),
        Err(_) => Err(serde::de::Error::custom(
            "error trying to deserialize boolean for task abort flag",
        )),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskState {
    pub requirements: TaskRequirements,
    // the list of resources this task is using
    pub allocation: ResourceAlloc,

    #[serde(
        deserialize_with = "deserialize_atomic_u64",
        serialize_with = "serialize_atomic_u64"
    )]
    pub last_seen: AtomicU64,

    #[serde(
        deserialize_with = "deserialize_atomic_bool",
        serialize_with = "serialize_atomic_bool"
    )]
    pub aborted: AtomicBool,
    // a timestamp indicating when this task was created
    pub creation_time: u64,
    pub context: String,
}

impl Clone for TaskState {
    fn clone(&self) -> Self {
        Self {
            requirements: self.requirements.clone(),
            allocation: self.allocation.clone(),
            last_seen: AtomicU64::new(self.last_seen.load(Ordering::Relaxed)),
            aborted: AtomicBool::new(self.aborted.load(Ordering::Relaxed)),
            creation_time: self.creation_time,
            context: self.context.clone(),
        }
    }
}

impl TaskState {
    pub fn new(requirements: TaskRequirements, allocation: ResourceAlloc, context: String) -> Self {
        let time: u64 = Utc::now().timestamp() as u64;
        TaskState {
            requirements,
            allocation,
            last_seen: AtomicU64::new(time),
            aborted: AtomicBool::new(false),
            creation_time: time,
            context,
        }
    }
    pub fn end_timestamp(&self) -> i64 {
        self.requirements
            .deadline
            .map_or(i64::MAX, |d| d.end_timestamp_secs())
    }

    /// compute whether a task is considered stalled
    ///
    /// using the value of [Settings::min_wait_time] seconds before now
    ///
    /// if [Settings::max_wait_time] is set, this function will check if the
    /// stalled task should be removed regardless its parent process remains
    /// active on the system.
    pub fn is_stalling(&self, scheduler_settings: &Settings) -> (bool, bool) {
        let min_wait_time = scheduler_settings.time_settings.min_wait_time;
        let max_wait_time = scheduler_settings.time_settings.max_wait_time;
        let now = Utc::now().timestamp() as u64;
        let is_stalled = now - min_wait_time > self.last_seen.load(Ordering::Relaxed);
        let must_be_removed = max_wait_time
            .map(|max| now - max > self.last_seen.load(Ordering::Relaxed))
            .unwrap_or(false);
        (is_stalled, must_be_removed)
    }

    pub fn last_seen(&self) -> u64 {
        self.last_seen.load(Ordering::Relaxed)
    }

    pub fn update_last_seen(&self) {
        self.last_seen
            .store(Utc::now().timestamp() as u64, Ordering::Relaxed);
    }

    pub fn aborted(&self) -> bool {
        self.aborted.load(Ordering::Relaxed)
    }

    pub fn abort(&self) {
        self.aborted.store(true, Ordering::Relaxed);
    }
}
