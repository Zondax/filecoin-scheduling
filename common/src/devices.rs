#[cfg(not(dummy_devices))]
use std::hash::{Hash, Hasher};

use rust_gpu_tools::opencl::GPUSelector;
#[cfg(not(dummy_devices))]
use rust_gpu_tools::opencl::{Device as ClDevice, DeviceUuid};

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone)]
pub struct Device {
    dev: ClDevice,
    pub selector: GPUSelector,
}

#[cfg(not(dummy_devices))]
impl Hash for Device {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        if let Some(uuid) = self.dev.uuid() {
            uuid.hash(hasher);
        } else if let Some(pci_id) = self.dev.pci_id() {
            pci_id.hash(hasher);
        } else {
            self.dev.name().hash(hasher);
        }
    }
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    memory: u64,
    selector: GPUSelector,
}

#[cfg(not(dummy_devices))]
impl Device {
    pub fn device_id(&self) -> GPUSelector {
        self.selector
    }
    pub fn device_uuid(&self) -> Option<DeviceUuid> {
        match self.selector {
            GPUSelector::Uuid(uuid) => Some(uuid),
            _ => None,
        }
    }

    pub fn device_pci_id(&self) -> Option<u32> {
        match self.selector {
            GPUSelector::PciId(pci) => Some(pci),
            _ => None,
        }
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
    pub fn device_id(&self) -> GPUSelector {
        self.selector
    }

    pub fn name(&self) -> String {
        format!("dummy_dev: {:?}", self.selector)
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
            .filter_map(|dev| {
                if let Some(uuid) = dev.uuid() {
                    Some((GPUSelector::Uuid(uuid), dev))
                } else {
                    dev.pci_id()
                        .map(|pci| (GPUSelector::PciId(pci), dev))
                        .or_else(|| {
                            tracing::error!(
                                "Device does not support the UUId nor PciId extensions"
                            );
                            None
                        })
                }
            })
            .map(|(selector, dev)| Device {
                dev: dev.clone(),
                selector,
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
    let gpu_devices = (0..3)
        .map(|i| Device {
            memory: 4,
            selector: GPUSelector::PciId(i as _),
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
