use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;

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
use jrpc_client::{Client as RpcClient, RpcClient as RpcClientTrait};

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
) -> Result<String, ClientError> {
    launch_scheduler_process()?;
    std::thread::sleep(Duration::from_millis(500));
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    let jrpc_client = RpcClient::new(SERVER_ADDRESS)?;

    rt.block_on(async {
        // We pass a simple string just for testing, in the real implementation it would be part of
        // the info contained in the task field
        let call_result = jrpc_client
            .schedule(format!("{}", client.pid))
            .await
            .map_err(|e| ClientError::Other(e.to_string()));
        call_result
    })?
}

fn launch_scheduler_process() -> Result<(), ClientError> {
    use nix::unistd::{fork, ForkResult};
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {}
        Ok(ForkResult::Child) => {
            scheduler_process();
        }
        Err(e) => return Err(ClientError::Other(e.to_string())),
    }
    Ok(())
}

fn scheduler_process() {
    // We are another process and can not redirect errors to the application
    // unless we use IPC communication like ipc-channel crate

    let mutex = GlobalMutex::new().unwrap();

    match mutex.try_lock() {
        Ok(_guard) => {
            let handler = std::thread::spawn(|| {
                let _ = run_scheduler();
            });
            // waiting for the scheduler thread allows us to keep the mutex locked throughout
            // the entire scheduler lifetime. If the Scheduler panics, the handler will return
            // immediately
            handler.join().unwrap();
        }
        _ => {}
    }
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
