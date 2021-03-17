mod error;
//mod ffi;
mod client;
mod config;
mod devices;
mod requests;
mod resource;
mod task;

pub use client::ClientToken;
pub use config::{ClientConfig, Config, SchedulerConfig};
pub use error::Error;
//pub use ffi::{FfiResourceAlloc, FfiResourceReq};
pub use devices::{list_devices, Device, Devices};
pub use requests::RequestMethod;
pub use resource::{ResourceAlloc, ResourceMemory, ResourceReq, ResourceType};
pub use task::{Deadline, Result, Task, TaskEstimations, TaskFunc, TaskRequirements, TaskResult};
