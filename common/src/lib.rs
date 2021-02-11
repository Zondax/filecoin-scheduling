mod error;
//mod ffi;
mod requests;
mod resource;
mod task;
mod client;

pub use client::ClientToken;
pub use error::Error;
//pub use ffi::{FfiResourceAlloc, FfiResourceReq};
pub use requests::RequestMethod;
pub use resource::{ResourceAlloc, ResourceReq};
pub use task::{Deadline, Task, TaskRequirements, TaskResult};
pub const SERVER_ADDRESS: &str = "127.0.0.1:5000";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
