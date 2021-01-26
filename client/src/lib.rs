use std::error::Error;
use std::fmt::Debug;
use std::sync::mpsc;
use std::time::Duration;

use tracing::info;

mod client;
mod error;
mod global_mutex;
mod jrpc_client;

pub use client::ClientToken;
pub use common::{ResourceAlloc, ResourceReq, Task};
pub use error::ClientError;
pub use scheduler::{run_scheduler, RequestMethod, SchedulerResponse};

use global_mutex::GlobalMutex;
use tokio::runtime::Runtime;

use common::SERVER_ADDRESS;
use jrpc_client::{Client as RpcClient, RpcClient as ClientTrait};

pub fn abort() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn register(pid: u32, client_id: u64) -> ClientToken {
    ClientToken::new(pid, client_id)
}

#[tracing::instrument(level = "info", skip(task))]
pub fn schedule<T: Debug + Copy + Clone>(
    client: ClientToken,
    task: Task<T>,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut runtime =
        tokio::runtime::Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    let mutex = GlobalMutex::new()?;

    let mut guard = mutex.try_lock();
    // The try lock api doesnt return error until a timeout is reached
    // we set a timeout of 1 ms as default, we can check this later
    match guard {
        Ok(locked) => {
            // The scheduler(and rpc server) is launched in its own thread
            // but we need to call fork() or something similar and give to it the
            // ownership of the mutex while it is still blocking other processes.
            // Rust will not allow us to move a mutex guard whose scope is limited to the place
            // where try_lock() is called, but it could be possible to have another atomic type as
            // the data protected by the mutex and set by the first process that adquire it, then
            // release it so the scheduler can have such mutex. in the meantime other processes
            // could read the mutex date and check if the atomic inside is set or not..the
            // scheduler is the only one that could modify it and clear the flag when exiting, this
            // may be done as part of the drop routine in the panic handler.
            // In the example: https://github.com/elast0ny/shared_memory-rs/blob/master/examples/mutex.rs
            // they do a cast from raw pointer to an AtomicU8, which means that it is safe. maybe we
            // do not even need a mutex? so the first process having access to the atomic will set
            // it, not allawing other processes to do so..meaning that the first one will be
            // allowed to start the scheduler process, then the latter will clear the atomic flag
            // when exiting
            run_scheduler().map_err(|e| ClientError::Other(e.to_string()))?;
            // Wait for the scheduler to set up
            std::thread::sleep(Duration::from_millis(500));
        }
        // In this case we know the scheduler is already running and do not have to wait for it to
        // be ready
        _ => {}
    }
    let jrpc_client = RpcClient::new(SERVER_ADDRESS)?;

    runtime.block_on(async {
        // We pass a simple string just for testing, in the real implementation it would be part of
        // the info contained in the task field
        jrpc_client
            .schedule(format!("process_id {}", client.pid))
            .await;
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
