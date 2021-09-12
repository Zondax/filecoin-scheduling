use rust_gpu_tools::opencl::UniqueId;
use serde::{de::Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug, Clone)]
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
    type Error = rust_gpu_tools::opencl::GPUError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(UniqueId::try_from(value)?))
    }
}

impl DeviceId {
    pub fn serialize_impl<S>(v: &Self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v.0 {
            UniqueId::PciId(id) => s.serialize_str(id.to_string().as_str()),
            UniqueId::Uuid(uuid) => s.serialize_str(uuid.to_string().as_str()),
        }
    }

    pub fn deserialize_impl<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(de)?;
        let inner =
            UniqueId::try_from(v.as_ref()).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(DeviceId(inner))
    }
}

impl Serialize for DeviceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::serialize_impl(self, serializer)
    }
}

impl<'de> Deserialize<'de> for DeviceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeviceId::deserialize_impl(deserializer)
    }
}
