use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize, Serializer};

use rust_gpu_tools::opencl::{DeviceUuid, GPUSelector};
use std::path::Path;
use tracing::error;

use common::TaskType;

const MAINTENANCE_INTERVAL: u64 = 10000;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    exec_time: u64,
    memory: u64,
    #[serde(
        deserialize_with = "deserialize_devices",
        serialize_with = "serialize_devices"
    )]
    devices: Vec<GPUSelector>,
    #[serde(deserialize_with = "TaskType::deserialize_with")]
    task_type: TaskType,
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        self.task_type
    }

    pub fn exec_time(&self) -> u64 {
        self.exec_time
    }

    pub fn devices(&self) -> Vec<GPUSelector> {
        self.devices.clone()
    }
}
pub fn deserialize_devices<'de, D>(de: D) -> Result<Vec<GPUSelector>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use std::convert::TryFrom;
    use std::str::FromStr;
    let s: Vec<String> = Vec::deserialize(de)?;
    let mut selectors = vec![];
    for id in s.iter() {
        match (
            DeviceUuid::try_from(id.as_str()),
            u32::from_str(id.as_str()),
        ) {
            (Ok(uuid), Err(_)) => selectors.push(GPUSelector::Uuid(uuid)),
            (Err(_), Ok(pci)) => selectors.push(GPUSelector::PciId(pci)),
            _ => {
                error!("unrecognized device id format: {}", id);
                return Err(serde::de::Error::custom("Unrecognized device id format"));
            }
        }
    }
    Ok(selectors)
}

fn serialize_devices<S>(v: &[GPUSelector], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let devices = v
        .iter()
        .map(|sel| match sel {
            GPUSelector::Uuid(uuid) => uuid.to_string(),
            GPUSelector::PciId(pci) => pci.to_string(),
            _ => Default::default(),
        })
        .collect::<Vec<String>>();
    s.collect_seq(devices)
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Service {
    address: String,
    /// interval in milliseconds. if present in the configuration file, creates a thread that performs some maintenance
    /// operations such as removing tasks that no longer exist in the system.
    pub maintenance_interval: Option<u64>,
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
            maintenance_interval: Some(MAINTENANCE_INTERVAL),
        };

        let time_settings = TimeSettings { min_wait_time: 120 };
        let exec_time = 60;
        let memory = 1024 * 32; // 32 kib
        let all_devices = common::list_devices()
            .gpu_devices()
            .iter()
            .map(|d| d.device_id())
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
                    task_i.devices = [all_devices[2]].to_vec();
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
