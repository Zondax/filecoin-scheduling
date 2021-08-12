use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

use std::path::Path;

use common::{DeviceId, TaskType};

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    devices: Vec<DeviceId>,
    #[serde(deserialize_with = "TaskType::deserialize_with")]
    task_type: TaskType,
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
    address: String,
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
            address: "127.0.0.1:5000".to_string(),
            maintenance_interval: Some(MAINTENANCE_INTERVAL),
            shutdown_timeout: Some(SHUTDOWN_TIMEOUT),
        };

        let time_settings = TimeSettings {
            min_wait_time: MIN_WAIT_TIME,
            max_wait_time: None,
        };
        let all_devices = common::list_devices()
            .gpu_devices()
            .iter()
            .map(|d| d.device_id())
            .collect::<Vec<_>>();
        let task = Task {
            devices: all_devices.clone(),
            task_type: TaskType::MerkleTree,
        };
        // create a setting with 3 task description
        let tasks_settings = (0..3)
            .map(|i| {
                let mut task_i = task.clone();
                task_i.task_type = match i {
                    1 => TaskType::WindowPost,
                    2 => TaskType::WinningPost,
                    _ => TaskType::MerkleTree,
                };
                if task_i.task_type == TaskType::WinningPost && cfg!(dummy_devices) {
                    task_i.devices = [all_devices[2].clone()].to_vec();
                }
                task_i
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
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
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
