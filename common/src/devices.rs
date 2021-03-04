use rust_gpu_tools::opencl;

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device(&'static opencl::Device);

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device(u32);

#[cfg(not(dummy_devices))]
impl Device {
    // TODO: Using the opencl address to the internal cl_device_id which is cast to an usize
    pub fn device_id(&self) -> usize {
        self.0.cl_device_id() as usize
    }

    pub fn name(&self) -> String {
        self.0.name()
    }

    pub fn memory(&self) -> u64 {
        self.0.memory()
    }

    pub fn brand(&self) -> opencl::Brand {
        self.0.brand()
    }

    pub fn bus_id(&self) -> Option<opencl::BusId> {
        self.0.bus_id()
    }
}

#[cfg(dummy_devices)]
impl Device {
    // TODO: Using the opencl address to the internal cl_device_id which is cast to an usize
    pub fn device_id(&self) -> usize {
        self.0 as usize
    }

    pub fn name(&self) -> String {
        format!("dummy_dev{}", self.0)
    }

    pub fn memory(&self) -> u64 {
        0
    }

    pub fn brand(&self) -> opencl::Brand {
        unimplemented!()
    }

    pub fn bus_id(&self) -> Option<opencl::BusId> {
        Some(self.0)
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
        .map(Device)
        .collect::<Vec<Device>>();
    #[cfg(dummy_devices)]
    let gpu_devices = (0..2).map(|i| Device(i)).collect::<Vec<Device>>();
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
