use rust_gpu_tools::opencl;

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    dev: &'static opencl::Device,
    memory: u64,
    id: usize,
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    memory: u64,
    id: usize,
}

#[cfg(not(dummy_devices))]
impl Device {
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

    pub fn bus_id(&self) -> Option<opencl::BusId> {
        self.dev.bus_id()
    }
}

#[cfg(dummy_devices)]
impl Device {
    pub fn device_id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> String {
        format!("dummy_dev{}", self.id)
    }

    pub fn memory(&self) -> u64 {
        self.memory
    }

    pub fn brand(&self) -> opencl::Brand {
        unimplemented!()
    }

    pub fn bus_id(&self) -> Option<opencl::BusId> {
        Some(self.id as _)
    }
}

#[derive(Debug)]
pub struct Devices {
    gpu_devices: Vec<Device>,
    num_cpus: usize,
    exclusive_gpus: Vec<usize>,
}

impl Devices {
    pub fn gpu_devices(&self) -> &[Device] {
        self.gpu_devices.as_ref()
    }

    pub fn exclusive_gpus(&self) -> &[usize] {
        self.exclusive_gpus.as_ref()
    }
}

/// Returns all the devices on the system
///
/// It includes the GPUs and the number of logical CPUs
pub fn list_devices() -> Devices {
    #[cfg(not(dummy_devices))]
    let gpu_devices = opencl::Device::all_iter()
        .map(|dev| {
            // Here we assume that every gpu device has a id value
            let id = dev.cl_device_id() as u16 as usize;
            let memory = dev.memory();
            Device { dev, memory, id }
        })
        .collect::<Vec<Device>>();

    #[cfg(dummy_devices)]
    let gpu_devices = (0..3)
        .map(|i| Device { memory: 4, id: i })
        .collect::<Vec<Device>>();

    #[cfg(not(dummy_devices))]
    let exclusive_gpus: Vec<usize> = vec![];
    #[cfg(dummy_devices)]
    let exclusive_gpus: Vec<usize> = vec![2];
    let num_cpus = num_cpus::get();
    Devices {
        gpu_devices,
        num_cpus,
        exclusive_gpus,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(dummy_devices)]
    #[test]
    fn check_devices() {
        let devices = list_devices();
        println!("DEVICES: {:?}", devices);
        let gpu2 = devices.gpu_devices[2].clone();
        assert!(devices
            .exclusive_gpus()
            .iter()
            .any(|&i| i == gpu2.device_id()));
        let exclusivegpu: Vec<Device> = devices
            .gpu_devices()
            .iter()
            .cloned()
            .filter(|dev| {
                devices
                    .exclusive_gpus()
                    .iter()
                    .any(|&i| i == dev.device_id())
            })
            .collect::<Vec<Device>>();
        assert_eq!(exclusivegpu.len(), 1);
        assert_eq!(exclusivegpu[0].device_id(), 2);
    }
}
