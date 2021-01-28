use std::fs::File;
use std::io::prelude::*;

use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;

mod client;
mod error;
mod global_mutex;
pub mod jrpc_client;

pub use crate::client::ClientToken;
pub use common::{Deadline, ResourceAlloc, ResourceReq, Task, TaskRequirements, TaskResult};
pub use error::ClientError;
pub use global_mutex::GlobalMutex;
pub use jrpc_client::*;
pub use scheduler::run_scheduler;

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
pub fn schedule<T: Debug + Clone>(
    client: ClientToken,
    task: Task<T>,
    _timeout: Duration,
) -> Result<String, ClientError> {
    launch_scheduler_process()?;
    std::thread::sleep(Duration::from_millis(500));
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    let jrpc_client = RpcClient::new(&format!("http://{}", SERVER_ADDRESS))?;

    let result = rt
        .block_on(async {
            // We pass a simple string just for testing, in the real implementation it would be part of
            // the info contained in the task field
            jrpc_client.schedule(format!("{}", client.pid)).await
        })
        .map_err(|e| ClientError::RpcError(e.to_string()))?;
    result.map_err(|e| ClientError::RpcError(e.to_string()))
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
            let _ = run_scheduler();
            // waiting for the scheduler thread allows us to keep the mutex locked throughout
            // the entire scheduler lifetime. If the Scheduler panics, the handler will return
            // immediately
        }
        _ => {}
    }
    mutex.release().unwrap();
    std::process::exit(0);
}

#[cfg(test)]
pub fn schedule_test<T: Debug + Clone>(
    client: ClientToken,
    _task: Task<T>,
    _timeout: Duration,
) -> Result<String, ClientError> {
    let handle = scheduler::run_scheduler_test().unwrap();
    std::thread::sleep(Duration::from_millis(500));
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    let jrpc_client = RpcClient::new(&format!("http://{}", SERVER_ADDRESS))?;

    let result = rt
        .block_on(async {
            // We pass a simple string just for testing, in the real implementation it would be part of
            // the info contained in the task field
            jrpc_client.schedule(format!("{}", client.pid)).await
        })
        .map_err(|e| ClientError::RpcError(e.to_string()))?;
    handle.close();
    result.map_err(|e| ClientError::RpcError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use scheduler::run_scheduler_test;

    #[test]
    fn calls_scheduler_one_process() {
        use chrono::{DateTime, NaiveDateTime, Utc};
        use rand::Rng;

        let task_fn =
            Box::new(|_data: Vec<ResourceAlloc>| TaskResult::Done(Ok("HelloWorld".to_string())));
        let req = ResourceReq {
            resource: "Gpu".to_string(),
            quantity: 2,
            preemptible: false,
        };
        let time_per_iteration = Duration::from_millis(10);
        let exec_time = Duration::from_millis(500);
        let start = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        let deadline = Deadline::new(start, end);
        let task = Task::new(task_fn, req, time_per_iteration, exec_time, deadline);

        let mut rng = rand::thread_rng();
        let pid: u32 = rng.gen();
        let client_id: u64 = rng.gen();
        let token = register(pid, client_id);

        let res = schedule_test(token, task, Default::default());
        println!("Result: {:?}", res);
        assert!(res.is_ok());
    }
}
