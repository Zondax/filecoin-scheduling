//use std::fs::File;
//use std::io::prelude::*;

use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;

mod global_mutex;
pub mod rpc_client;

pub use common::{
    list_devices, ClientToken, Deadline, Devices, Error as ClientError, ResourceAlloc, ResourceReq,
    Task, TaskRequirements, TaskResult,
};
pub use global_mutex::GlobalMutex;
pub use rpc_client::*;
pub use scheduler::run_scheduler;

use tokio::runtime::Runtime;

use rpc_client::{Client as RpcClient, RpcClient as RpcClientTrait};

const SERVER_ADDRESS: &str = "127.0.0.1:5000";

// The initial idea for testing addresses was using std::net::TcpListener::bind(x.x.x.x:0)
// that returns a random port that is not being used, but considering that we may have multiple
// processes running on tests, having a static address is the best approach
const TEST_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn server_address() -> String {
    if !cfg!(test) {
        // This can change so the address might come from a configuration file along other settings
        SERVER_ADDRESS.to_string()
    } else {
        TEST_SERVER_ADDRESS.to_string()
    }
}

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
    timeout: Duration,
) -> Result<(), ClientError> {
    let address = server_address();
    let mut jrpc_client = RpcClient::new(&address)?;
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;

    let result = rt.block_on(async {
        let allocation = if jrpc_client.check_server().await.is_ok() {
            wait_allocation(&mut jrpc_client, client, task.task_req, timeout).await
        } else {
            launch_scheduler_process(address)?;
            std::thread::sleep(Duration::from_millis(500));
            wait_allocation(&mut jrpc_client, client, task.task_req, timeout).await
        };
        // TODO: implement the next parts of this function
        allocation
    });
    result.map(|_| ())
}

async fn wait_allocation(
    rpc_client: &mut RpcClient,
    client: ClientToken,
    requirements: TaskRequirements,
    timeout: std::time::Duration,
) -> Result<ResourceAlloc, ClientError> {
    tokio::select! {
        _ = tokio::time::timeout(timeout, futures::future::pending::<()>()) => {
            return Err(ClientError::Timeout);
        }
        call_res = async {
            loop {
                match rpc_client.wait_allocation(client, requirements.clone()).await {
                    Ok(Ok(dev)) => {
                        if let Some(alloc) = dev {
                            return Ok(alloc);
                        } else {
                            // There are not available resources at this point so we have to try
                            // again.
                            continue
                        }
                    }
                    Ok(Err(e)) => return Err(e),
                    Err(e) => return Err(ClientError::RpcError(e.to_string())),
                }
            }
        } => {
            return call_res;
        }
    }
}

#[allow(dead_code)]
fn launch_scheduler_process(address: String) -> Result<(), ClientError> {
    use nix::unistd::{fork, ForkResult};
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => Ok(()),
        Ok(ForkResult::Child) => {
            let mutex = GlobalMutex::new()?;
            if let Ok(_guard) = mutex.try_lock() {
                let _ = run_scheduler(&address);
                mutex.release().unwrap();
            }
            Ok(())
        }
        Err(e) => Err(ClientError::Other(e.to_string())),
    }
}

pub fn list_all_resources() -> Devices {
    common::list_devices()
}

pub fn list_allocations() -> Result<Vec<u32>, ClientError> {
    let jrpc_client = RpcClient::new(&server_address())?;
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    rt.block_on(async { jrpc_client.list_allocations().await })
        .map_err(|e| ClientError::Other(e.to_string()))
}

pub fn release(alloc: ResourceAlloc) -> Result<(), ClientError> {
    let jrpc_client = RpcClient::new(&server_address())?;
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    rt.block_on(async { jrpc_client.release(alloc).await })
        .map_err(|e| ClientError::RpcError(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task<T>(t: impl Fn(Vec<ResourceAlloc>) -> TaskResult<T> + 'static) -> Task<T> {
        use chrono::{DateTime, NaiveDateTime, Utc};

        let task_fn = Box::new(t);
        let req = ResourceReq {
            resource: common::ResourceType::Gpu,
            quantity: 2,
            preemptible: false,
        };
        let time_per_iteration = Duration::from_millis(10);
        let exec_time = Duration::from_millis(500);
        let start = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        let deadline = Deadline::new(start, end);
        Task::new(
            task_fn,
            vec![req.clone()],
            time_per_iteration,
            exec_time,
            deadline,
        )
    }

    #[test]
    fn calls_scheduler_one_process() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let pid: u32 = rng.gen();
        let client_id: u64 = rng.gen();
        let token = register(pid, client_id);

        let handle = scheduler::spawn_scheduler_with_handler(&server_address()).unwrap();

        let task = task(|_data: Vec<ResourceAlloc>| TaskResult::Done(Ok("HelloWorld".to_string())));

        let res = schedule_one_of(token, task, Default::default());
        // Accept just this type of error
        if let Err(e) = res {
            assert_eq!(e, ClientError::Timeout);
        }
        handle.close();
    }

    #[test]
    fn allocation_test() {
        let address = "127.0.0.1:6000".to_string();
        let mut jrpc_client = RpcClient::new(&address).unwrap();
        let handle = scheduler::spawn_scheduler_with_handler(&address).unwrap();
        let rt = Runtime::new().unwrap();
        let client = register(10, 25);
        let task = task(|_data: Vec<ResourceAlloc>| TaskResult::Done(Ok("HelloWorld".to_string())));

        let result = rt.block_on(wait_allocation(
            &mut jrpc_client,
            client,
            task.task_req,
            // How much are we eager to wait for a resource to be allocated for us?
            std::time::Duration::from_millis(3000),
        ));

        // If there are not gpu devices attached to the system we expect a timeout error
        if list_all_resources().gpu_devices().is_empty() {
            assert_eq!(result.unwrap_err(), ClientError::Timeout);
        } else {
            assert!(result.is_ok());
        }

        handle.close();
    }

    #[test]
    fn release_test() {
        // This test only check communication and well formed param parsing
        let address = "127.0.0.1:7000".to_string();
        let jrpc_client = RpcClient::new(&address).unwrap();
        let handle = scheduler::spawn_scheduler_with_handler(&address).unwrap();
        let rt = Runtime::new().unwrap();
        let res_req = ResourceReq {
            resource: common::ResourceType::Gpu,
            quantity: 1,
            preemptible: true,
        };
        let res_alloc = ResourceAlloc {
            resource: res_req,
            resource_id: 0,
        };

        let res = rt
            .block_on(async { jrpc_client.release(res_alloc).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))
            .unwrap();

        assert!(res.is_ok());

        handle.close();
    }
}
