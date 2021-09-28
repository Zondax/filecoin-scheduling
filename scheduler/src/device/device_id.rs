use rust_gpu_tools::UniqueId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct DeviceId(pub UniqueId);

impl PartialEq for DeviceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for DeviceId {}

impl Hash for DeviceId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.0 {
            UniqueId::PciId(id) => id.hash(state),
            UniqueId::Uuid(uuid) => uuid.hash(state),
        }
    }
}

impl Deref for DeviceId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for DeviceId {
    type Error = rust_gpu_tools::GPUError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(UniqueId::try_from(value)?))
    }
}

impl TryFrom<String> for DeviceId {
    type Error = rust_gpu_tools::GPUError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(UniqueId::try_from(value.as_str())?))
    }
}

impl From<DeviceId> for String {
    fn from(id: DeviceId) -> Self {
        id.0.to_string()
    }
}
