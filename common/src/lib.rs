mod error;
//mod ffi;
mod client;
mod requests;
mod resource;
mod task;

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
        let a = 2;
        let b = 2;
        assert_eq!(a + b, 4);
    }
}
