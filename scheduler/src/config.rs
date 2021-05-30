use config::{Config, ConfigError, File};

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::path::Path;

use common::TaskType;

pub trait DeserializeWith: Sized {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl DeserializeWith for TaskType {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    exec_time: u64,
    memory: u64,
    devices: Vec<String>,
    #[serde(deserialize_with = "TaskType::deserialize_with")]
    task_type: TaskType,
}

impl Task {
    pub fn get_task_type(&self) -> TaskType {
        self.task_type
    }

    pub fn get_task_exec_time(&self) -> u64 {
        self.exec_time
    }

    pub fn get_devices(&self) -> Vec<String> {
        self.devices.clone()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Service {
    address: String,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeSettings {
    pub min_wait_time: u64,
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
        };

        let time_settings = TimeSettings { min_wait_time: 60 };
        let exec_time = 60;
        let memory = 1024 * 32; // 32 kib
        let all_devices = common::list_devices()
            .gpu_devices()
            .iter()
            .map(|dev| dev.device_id().map(|d| d.to_string()).unwrap_or_default())
            .collect::<Vec<_>>();
        let task = Task {
            exec_time,
            memory,
            devices: all_devices.clone(),
            task_type: TaskType::MerkleProof,
        };
        // create a setting with 3 task description
        let tasks_settings = (0..3)
            .map(|i| {
                let mut task_i = task.clone();
                task_i.task_type = match i {
                    1 => TaskType::WindowPost,
                    2 => TaskType::WinningPost,
                    _ => TaskType::MerkleProof,
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
            let s = Settings::default();
            let toml = toml::to_string(&s).map_err(|e| {
                ConfigError::Message(format!("Error generating toml date {}", e.to_string()))
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
