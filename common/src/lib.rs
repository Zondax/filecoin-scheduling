mod error;
//mod ffi;
mod resource;
mod task;
pub use error::Error;
//pub use ffi::{FfiResourceAlloc, FfiResourceReq};
pub use resource::{ResourceAlloc, ResourceReq};
pub use task::{Deadline, Task};

pub const SERVER_ADDRESS: &'static str = "127.0.0.7:5000";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
