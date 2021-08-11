use std::hash::{Hash, Hasher};

use crate::DeviceId;

use rust_gpu_tools::opencl::Device as ClDevice;

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone)]
pub struct Device {
    dev: ClDevice,
    pub id: DeviceId,
}

#[cfg(not(dummy_devices))]
impl Hash for Device {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        if let Some(uuid) = self.dev.uuid() {
            uuid.hash(hasher);
        } else {
            self.dev.name().hash(hasher);
        }
        let pci_id = self.dev.pci_id();
        pci_id.hash(hasher);
    }
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    memory: u64,
    id: DeviceId,
}

#[cfg(not(dummy_devices))]
impl Device {
    pub fn device_id(&self) -> DeviceId {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.dev.name()
    }

    pub fn memory(&self) -> u64 {
        self.dev.memory()
    }

    pub fn get_inner(&self) -> ClDevice {
        self.dev.clone()
    }
}
#[cfg(dummy_devices)]
impl Device {
    pub fn device_id(&self) -> DeviceId {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.id.to_string()
    }

    pub fn memory(&self) -> u64 {
        self.memory
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Devices {
    gpu_devices: Vec<Device>,
    num_cpus: usize,
}

impl Devices {
    pub fn gpu_devices(&self) -> &[Device] {
        self.gpu_devices.as_ref()
    }
}

/// Returns all the devices on the system
///
/// It includes the GPUs and the number of logical CPUs
#[cfg(not(dummy_devices))]
pub fn list_devices() -> Devices {
    let gpu_devices = {
        ClDevice::all()
            .into_iter()
            .map(|dev| {
                let unique_id = dev.unique_id();
                (DeviceId(unique_id), dev)
            })
            .map(|(id, dev)| Device {
                dev: dev.clone(),
                id,
            })
            .collect::<Vec<Device>>()
    };
    let num_cpus = num_cpus::get();

    Devices {
        gpu_devices,
        num_cpus,
    }
}

#[cfg(dummy_devices)]
pub fn list_devices() -> Devices {
    use rust_gpu_tools::opencl::UniqueId;
    use std::convert::TryFrom;

    let gpu_devices = (0..3)
        .map(|i| Device {
            memory: 4,
            id: DeviceId(UniqueId::try_from(format!("00:0{}", i).as_str()).unwrap()),
        })
        .collect::<Vec<Device>>();

    let num_cpus = num_cpus::get();
    Devices {
        gpu_devices,
        num_cpus,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_devices() {
        use crate::{list_devices, Devices};

        let devices = list_devices();
        println!(
            "DEVICES: {:?} len {}",
            devices,
            std::mem::size_of::<Devices>()
        );
    }
}
