use rust_gpu_tools::opencl;

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    dev: &'static opencl::Device,
    memory: u64,
    bus_id: u32,
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    memory: u64,
    bus_id: u32,
}

#[cfg(not(dummy_devices))]
impl Device {
    // TODO: Using the opencl address to the internal cl_device_id which is cast to an usize
    pub fn device_id(&self) -> usize {
        self.dev.cl_device_id() as usize
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

    pub fn bus_id(&self) -> opencl::BusId {
        self.bus_id
    }
}

#[cfg(dummy_devices)]
impl Device {
    pub fn device_id(&self) -> usize {
        self.bus_id as usize
    }

    pub fn name(&self) -> String {
        format!("dummy_dev{}", self.bus_id)
    }

    pub fn memory(&self) -> u64 {
        self.memory
    }

    pub fn brand(&self) -> opencl::Brand {
        unimplemented!()
    }

    pub fn bus_id(&self) -> opencl::BusId {
        self.bus_id
    }
}

#[derive(Debug)]
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
    let gpu_devices = opencl::Device::all_iter()
        .filter_map(|dev| {
            // Here we assume that every gpu device has a bus_id value
            if let Some(bus_id) = dev.bus_id() {
                let memory = dev.memory();
                Some(Device {
                    dev,
                    memory,
                    bus_id,
                })
            } else {
                None
            }
        })
        .collect::<Vec<Device>>();

    #[cfg(dummy_devices)]
    let gpu_devices = (0..2)
        .map(|i| Device {
            memory: 4,
            bus_id: i,
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
    use super::*;

    #[test]
    fn check_devices() {
        let devices = list_devices();
        println!("DEVICES: {:?}", devices);
    }
}
