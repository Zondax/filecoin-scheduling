use rust_gpu_tools::opencl;
#[cfg(dummy_devices)]
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Device {
    dev: opencl::Device,
    memory: u64,
    // the device uuid
    id: Option<opencl::DeviceUuid>,
}

#[cfg(not(dummy_devices))]
impl Hash for Device {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
        self.dev.name().hash(hasher);
    }
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Device {
    memory: u64,
    id: Option<opencl::DeviceUuid>,
}

#[cfg(not(dummy_devices))]
impl Device {
    pub fn device_id(&self) -> Option<opencl::DeviceUuid> {
        self.id
    }

    pub fn device_pci_id(&self) -> Option<u32> {
        self.dev.pci_id()
    }

    pub fn name(&self) -> String {
        self.dev.name()
    }

    pub fn memory(&self) -> u64 {
        self.memory
    }

    pub fn brand(&self) -> opencl::Brand {
        self.dev.brand()
    }

    pub fn get_inner(&self) -> opencl::Device {
        self.dev.clone()
    }
}

#[cfg(dummy_devices)]
impl Device {
    pub fn device_id(&self) -> Option<opencl::DeviceUuid> {
        self.id
    }

    pub fn name(&self) -> String {
        format!("dummy_dev: {}", self.id.unwrap())
    }

    pub fn memory(&self) -> u64 {
        self.memory
    }

    pub fn brand(&self) -> opencl::Brand {
        unimplemented!()
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
pub fn list_devices() -> Devices {
    #[cfg(not(dummy_devices))]
    let gpu_devices = {
        let devs = opencl::Device::all();
        devs.into_iter()
            .map(|dev| {
                let memory = dev.memory();
                Device {
                    dev: dev.clone(),
                    memory,
                    id: dev.uuid(),
                }
            })
            .collect::<Vec<Device>>()
    };

    #[cfg(dummy_devices)]
    let gpu_devices = (0..3)
        .map(|i| {
            let uuid =
                opencl::DeviceUuid::try_from(format!("00000000-0000-0000-0000-00000000000{:x}", i))
                    .unwrap();
            Device {
                memory: 4,
                id: Some(uuid),
            }
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

    #[cfg(dummy_devices)]
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
