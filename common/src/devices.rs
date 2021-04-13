use fil_ocl_core::{ffi, get_device_info_raw, util, DeviceId as DeviceIdCore, Error};
use rust_gpu_tools::opencl;
use std::mem::{ManuallyDrop, MaybeUninit};

const CL_DEVICE_PCI_SLOT_ID_NV: ffi::cl_uint = 0x4009;
const CL_DEVICE_TOPOLOGY_AMD: ffi::cl_uint = 0x4037;
const CL_DEVICE_TOPOLOGY_TYPE_PCIE_AMD: ffi::cl_uint = 1;

#[repr(C)]
struct Raw {
    rtype: u32,
    data: [u32; 5],
}

#[repr(C)]
struct Pcie {
    rtype: u32,
    data: [u32; 17],
    bus: u8,
    device: u8,
    function: u8,
}

#[repr(C)]
union cl_device_topology_amd {
    raw: ManuallyDrop<Raw>,
    pcie: ManuallyDrop<Pcie>,
}

fn get_device_unique_id_nv(dev: &DeviceIdCore) -> Result<u32, Error> {
    let result = get_device_info_raw(dev, CL_DEVICE_PCI_SLOT_ID_NV as _)?;
    Ok(unsafe { util::bytes_into::<u32>(result)? })
}

#[allow(clippy::uninit_assumed_init)]
fn get_device_unique_id_amd(dev: &DeviceIdCore) -> Result<u32, Error> {
    use fil_ocl_core::ClDeviceIdPtr;

    let mut topology: cl_device_topology_amd = unsafe { MaybeUninit::uninit().assume_init() };
    let size = std::mem::size_of::<cl_device_topology_amd>();
    let errcode = unsafe {
        ffi::clGetDeviceInfo(
            dev.as_ptr() as _,
            CL_DEVICE_TOPOLOGY_AMD as _,
            size as _,
            &mut topology as *mut _ as *mut _,
            std::ptr::null_mut(),
        )
    };

    if errcode < 0 {
        return Err(fil_ocl_core::Error::from(
            "<unavailable (CL_INVALID_OPERATION)>",
        ));
    }
    unsafe {
        if topology.raw.rtype == CL_DEVICE_TOPOLOGY_TYPE_PCIE_AMD {
            let device = topology.pcie.device as u32;
            let bus = topology.pcie.bus as u32;
            let func = topology.pcie.function as u32;

            Ok((device << 16) | (bus << 8) | func)
        } else {
            Err(fil_ocl_core::Error::from(
                "<unavailable (CL_DEVICE_TOPOLOGY_TYPE_PCIE_AMD)>",
            ))
        }
    }
}

fn hash(id: Option<u32>, name: String) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut s = DefaultHasher::new();
    id.hash(&mut s);
    name.hash(&mut s);
    s.finish()
}

#[cfg(not(dummy_devices))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Device {
    dev: opencl::Device,
    memory: u64,
    id: u64,
}

#[cfg(dummy_devices)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Device {
    memory: u64,
    id: u64,
}

#[cfg(not(dummy_devices))]
impl Device {
    pub fn device_id(&self) -> u64 {
        self.id
    }

    pub fn device_unique_id(&self) -> Result<Option<u32>, Error> {
        let res = match self.brand() {
            opencl::Brand::Nvidia => Some(get_device_unique_id_nv(self.dev.device.as_core())?),
            opencl::Brand::Amd => Some(get_device_unique_id_amd(self.dev.device.as_core())?),
            _ => None,
        };
        Ok(res)
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

    pub fn get_inner(&self) -> opencl::Device {
        self.dev.clone()
    }
}

#[cfg(dummy_devices)]
impl Device {
    pub fn device_id(&self) -> u64 {
        self.id
    }

    pub fn device_unique_id(&self) -> Result<Option<u32>, Error> {
        Ok(Some(self.id as _))
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
                let unique_id = match dev.brand() {
                    opencl::Brand::Nvidia => get_device_unique_id_nv(dev.device.as_core()).ok(),
                    opencl::Brand::Amd => get_device_unique_id_amd(dev.device.as_core()).ok(),
                    _ => None,
                };

                let name = dev.name();
                let id = hash(unique_id, name);
                Device {
                    dev: dev.clone(),
                    memory,
                    id,
                }
            })
            .collect::<Vec<Device>>()
    };

    #[cfg(dummy_devices)]
    let gpu_devices = (0..3)
        .map(|i| Device {
            memory: 4,
            id: i as u64,
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

    #[cfg(dummy_devices)]
    #[test]
    fn check_devices() {
        let devices = list_devices();
        println!(
            "DEVICES: {:?} len {}",
            devices,
            std::mem::size_of::<Devices>()
        );
    }

    #[test]
    fn unique_id() {
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};

        let set = Arc::new(Mutex::new(HashSet::new()));
        let num_devices = list_devices().gpu_devices().len();

        let mut handlers = vec![];
        for _ in 0..5 {
            let set = Arc::clone(&set);
            handlers.push(std::thread::spawn(|| {
                list_devices().gpu_devices().iter().for_each(move |dev| {
                    set.lock().unwrap().insert(dev.device_id());
                });
            }));
        }

        for handle in handlers.into_iter() {
            handle.join().unwrap()
        }
        assert_eq!(set.lock().unwrap().len(), num_devices);
    }
}
