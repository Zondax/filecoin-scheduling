mod error;
//mod ffi;
mod client;
mod devices;
mod requests;
mod resource;
mod task;

pub use client::ClientToken;
pub use error::Error;
//pub use ffi::{FfiResourceAlloc, FfiResourceReq};
pub use devices::{list_devices, Device, Devices};
pub use requests::RequestMethod;
pub use resource::{ResourceAlloc, ResourceReq, ResourceType};
pub use task::{Deadline, Task, TaskRequirements, TaskResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = 2;
        let b = 2;
        assert_eq!(a + b, 4);
    }
}
