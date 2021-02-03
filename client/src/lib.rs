//use std::fs::File;
//use std::io::prelude::*;

use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;

mod client;
mod global_mutex;
pub mod rpc_client;

pub use crate::client::ClientToken;
pub use common::{Deadline, ResourceAlloc, ResourceReq, Task, TaskRequirements, TaskResult};
pub use global_mutex::GlobalMutex;
pub use rpc_client::*;
pub use scheduler::run_scheduler;

use jsonrpc_client::Error as RpcError;
use tokio::runtime::Runtime;

use common::Error as ClientError;
use common::SERVER_ADDRESS;
use rpc_client::{Client as RpcClient, RpcClient as RpcClientTrait};

pub fn abort() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn register(pid: u32, client_id: u64) -> ClientToken {
    ClientToken::new(pid, client_id)
}

#[tracing::instrument(level = "info", skip(task))]
pub fn schedule_one_of<T: Debug + Clone>(
    client: ClientToken,
    task: Task<T>,
    _timeout: Duration,
) -> Result<(), ClientError> {
    let jrpc_client = RpcClient::new(&format!("http://{}", SERVER_ADDRESS))?;
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;

    let result = rt.block_on(async { jrpc_client.schedule_one_of(task.task_req.clone()).await });

    if let Ok(r) = result {
        return r.map(|_| ());
    }

    let e = result.unwrap_err();
    // We assume here that the returned error is of type reqwest::Error because of the feature we enabled
    // here for the jsonrpc_client crate, if we use surf or another feature for such crate the code
    // bellow will not work
    if let RpcError::Client(ref e) = e {
        // A connection type error that means the scheduler is offline
        if e.is_connect() || e.is_timeout() {
            #[cfg(not(test))]
            launch_scheduler_process()?;

            #[cfg(test)]
            let handle = scheduler::spawn_scheduler_with_handler().unwrap();

            std::thread::sleep(Duration::from_millis(500));

            let result = rt.block_on(async { jrpc_client.schedule_one_of(task.task_req).await });

            #[cfg(test)]
            handle.close();

            return result
                .map_err(|e| ClientError::RpcError(e.to_string()))?
                .map(|_| ());
        }
    }
    Err(ClientError::Other(e.to_string()))
}

fn launch_scheduler_process() -> Result<(), ClientError> {
    use nix::unistd::{fork, ForkResult};
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => Ok(()),
        Ok(ForkResult::Child) => {
            let mutex = GlobalMutex::new()?;
            if let Ok(_guard) = mutex.try_lock() {
                let _ = run_scheduler();
                mutex.release().unwrap();
            }
            Ok(())
        }
        Err(e) => Err(ClientError::Other(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let task = Task::new(
            task_fn,
            vec![req.clone()],
            time_per_iteration,
            exec_time,
            deadline,
        );

        let mut rng = rand::thread_rng();
        let pid: u32 = rng.gen();
        let client_id: u64 = rng.gen();
        let token = register(pid, client_id);

        let res = schedule_one_of(token, task, Default::default());
        assert!(res.is_ok());
        //assert_eq!(res.unwrap().parse::<u32>().unwrap(), pid);
    }
}
