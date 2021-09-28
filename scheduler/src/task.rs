use serde::{de::Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use crate::{ResourceAlloc, TaskRequirements};
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
    pub fn end_timestamp(&self) -> i64 {
        self.requirements
            .deadline
            .map_or(i64::MAX, |d| d.end_timestamp_secs())
    }
}
