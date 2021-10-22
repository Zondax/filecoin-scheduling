use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::{DeviceId, TaskType};

/// Define the interval in milliseconds
/// after which the maintenance thread
/// will perform a maintenance cycle
const MAINTENANCE_INTERVAL: u64 = 2000;

/// define the time in seconds after which
/// the maintenance thread will close the
/// scheduler if it has been inactive in the
/// sense that there are neither pending jobs
/// nor requests from clients
const SHUTDOWN_TIMEOUT: u64 = 300;

const MIN_WAIT_TIME: u64 = 120;
// default deadline for merkeltree tasks
const DEFAULT_DEADLINE: u64 = 1500;
// Constant that defines the winning_post deadline and timeout
const WINNING_POST_DEADLINE: u64 = 15;
// Constant that defines the window_post deadline and timeout
const WINDOW_POST_DEADLINE: u64 = 900;

// Server address
const SERVER_ADDRESS: &str = "127.0.0.1:5000";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub task_type: TaskType,
    pub devices: Vec<DeviceId>,
    pub timeout: u64,
    pub deadline: u64,
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        self.task_type
    }

    pub fn devices(&self) -> Vec<DeviceId> {
        self.devices.clone()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Service {
    pub address: String,
    /// interval in milliseconds. if present in the configuration file, creates a thread that performs some maintenance
    /// operations such as removing tasks that no longer exist in the system or automatic shutdown
    /// if there are not more tasks or requests.
    pub maintenance_interval: Option<u64>,
    /// Time in seconds until the service should close itself if there are not more clients or
    /// requests. This is done only if the [Service::maintenance_interval] setting is set in the
    /// configuration.
    pub shutdown_timeout: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeSettings {
    /// time in seconds after which a task is considered stalled
    pub min_wait_time: u64,
    /// time in seconds after which a task that is stalling would be removed
    /// this setting just remove the job from the scheduler internal state,
    /// there is not any warranty on the state of the resources the task was using.
    /// this is undefined behavior and is not enable by default.
    pub max_wait_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub tasks_settings: Vec<Task>,
    pub service: Service,
    pub time_settings: TimeSettings,
}

impl Default for Settings {
    fn default() -> Self {
        let service = Service {
            address: SERVER_ADDRESS.to_string(),
            maintenance_interval: Some(MAINTENANCE_INTERVAL),
            shutdown_timeout: Some(SHUTDOWN_TIMEOUT),
        };

        let time_settings = TimeSettings {
            min_wait_time: MIN_WAIT_TIME,
            max_wait_time: None,
        };
        let all_devices = crate::list_devices()
            .gpu_devices()
            .iter()
            .map(|d| d.device_id())
            .collect::<Vec<_>>();
        // create a setting with 3 tasks description
        let tasks_settings = (0..3)
            .map(|i| {
                let (task_type, deadline) = match i {
                    1 => (TaskType::WinningPost, WINNING_POST_DEADLINE),
                    2 => (TaskType::WindowPost, WINDOW_POST_DEADLINE),
                    _ => (TaskType::MerkleTree, DEFAULT_DEADLINE),
                };
                let mut task = Task {
                    task_type,
                    devices: all_devices.clone(),
                    timeout: deadline,
                    deadline,
                };
                if task.task_type == TaskType::WinningPost && cfg!(dummy_devices) {
                    task.devices = [all_devices[2].clone()].to_vec();
                }
                task
            })
            .collect::<Vec<_>>();

        Settings {
            tasks_settings,
            service,
            time_settings,
        }
    }
}

impl Settings {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
        if path.as_ref().exists() {
            let mut s = Config::new();
            s.merge(File::with_name(path.as_ref().to_str().ok_or_else(
                || ConfigError::Message("Invalid config path".to_string()),
            )?))?;
            s.try_into()
        } else {
            let s = Self::default();
            let toml = toml::to_string(&s).map_err(|e| {
                ConfigError::Message(format!("Error generating toml file: {}", e.to_string()))
            })?;
            std::fs::write(&path, &toml).map_err(|e| {
                ConfigError::Message(format!(
                    "Can not create default configuration file {}",
                    e.to_string()
                ))
            })?;
            Ok(s)
        }
    }

    // match all the devices that were assigned to task with type taskType
    // returns None if there are not.
    pub fn devices_for_task(&self, task_type: Option<TaskType>) -> Option<Vec<DeviceId>> {
        if let Some(this_task) = task_type {
            for task in self.tasks_settings.iter() {
                let devices = task.devices();
                if task.task_type() == this_task && !devices.is_empty() {
                    return Some(devices);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_config() {
        let path = "/tmp/test.toml";
        let settings = Settings::new(path).unwrap();
        let set = {
            let mut s = Config::new();
            s.merge(File::with_name(path)).unwrap();
            s.try_into().unwrap()
        };
        assert_eq!(settings, set);
    }
}
