use std::time::Duration;
use tracing::{debug, error, info, warn};

mod global_mutex;
pub mod rpc_client;

pub use common::{
    list_devices, ClientToken, Deadline, Devices, Error as ClientError, ResourceAlloc,
    ResourceMemory, ResourceReq, ResourceType, TaskEstimations, TaskFunc, TaskReqBuilder,
    TaskRequirements, TaskResult,
};
pub use global_mutex::GlobalMutex;
pub use rpc_client::*;
use scheduler::run_scheduler;
pub use scheduler::spawn_scheduler_with_handler;

use tokio::runtime::Runtime;

use rpc_client::{Client, RpcClient};

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

#[tracing::instrument(level = "info")]
pub fn abort(_client: ClientToken) -> Result<(), ClientError> {
    Ok(())
}

#[tracing::instrument(level = "info", skip(pid, client_id))]
pub fn register(pid: u32, client_id: u64) -> Result<Client, ClientError> {
    info!("new client: {} - with process_id: {}", client_id, pid);
    let token = ClientToken::new(pid, client_id);
    // TODO: Here we look for the config file and get the address from there as other params as
    // well
    Client::new(&server_address(), token)
}

/// Schedules a task
///
/// The scheduler would pick up one of the resource requirements the client listed
/// according to the current resource usage in terms of memory and priorities that this task and
/// the ones already running have.
///
/// # Arguments:
/// * `client` - The client identifier
/// * `task_func` - The task functions object that implements [TaskFunc] trait
/// * `req` - The task requirements, what is needed for executing this task. It also gives some
/// information about execution times, deadlines and resource requirements. If __None__ the task
/// would be executed inmediately without the intervention of the scheduler service. Otherwise the
/// task is scheduled on the resource that best fit the requirements. The task execution
/// will be controlled by the scheduler service.
/// * `timeout` - Indicates how much the client is able to wait for the task to be scheduled. It is
/// possible that the client have to wait for resources to be freed when other task are done. If it expires and Error would be returned indicating it was
/// the case.
#[tracing::instrument(level = "info", skip(timeout, task_func, req, client))]
pub fn schedule_one_of<T>(
    client: Client,
    task_func: &mut dyn TaskFunc<TaskOutput = T>,
    req: Option<TaskRequirements>,
    timeout: Duration,
) -> Result<T, ClientError> {
    let address = server_address();

    if req.is_none() {
        return execute_without_scheduler(task_func);
    }

    let req = req.unwrap();

    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;

    rt.block_on(async {
        let allocation = if client.check_server().await.is_ok() {
            tracing::info!("Scheduler running ");
            wait_allocation(&client, req.clone(), timeout).await
        } else {
            launch_scheduler_process(address)?;
            std::thread::sleep(Duration::from_millis(500));
            wait_allocation(&client, req, timeout).await
        };

        match allocation {
            Err(e) => Err(e),
            Ok(alloc) => {
                let result = execute_task(&client, timeout, task_func, &alloc).await;
                release(&client).await?;
                result
            }
        }
    })
}

fn execute_without_scheduler<T>(
    task_func: &mut dyn TaskFunc<TaskOutput = T>,
) -> Result<T, ClientError> {
    task_func
        .init(None)
        .map_err(|e| ClientError::Other(e.to_string()))?;
    let mut cont = TaskResult::Continue;
    while cont == TaskResult::Continue {
        cont = task_func
            .task(None)
            .map_err(|e| ClientError::Other(e.to_string()))?;
    }
    task_func
        .end(None)
        .map_err(|e| ClientError::Other(e.to_string()))
}

#[tracing::instrument(level = "info", skip(client, timeout, task, alloc))]
async fn execute_task<'a, T>(
    client: &Client,
    timeout: Duration,
    task: &mut dyn TaskFunc<TaskOutput = T>,
    alloc: &ResourceAlloc,
) -> Result<T, ClientError> {
    // Initialize user resources
    task.init(Some(alloc))
        .map_err(|e| ClientError::ClientInit(e.to_string()))?;
    let mut cont = TaskResult::Continue;
    while cont == TaskResult::Continue {
        while wait_preemptive(client, timeout).await? {
            tokio::time::sleep(Duration::from_secs(1)).await
        }
        debug!(
            "Client {} task iteration completed",
            client.token.process_id()
        );
        cont = task
            .task(Some(alloc))
            .map_err(|e| ClientError::Other(e.to_string()))?;
        release_preemptive(client).await?;
    }

    task.end(Some(alloc))
        .map_err(|e| ClientError::ClientEnd(e.to_string()))
}

#[tracing::instrument(level = "info", skip(client))]
async fn wait_preemptive(client: &Client, _timeout: Duration) -> Result<bool, ClientError> {
    info!("client: {} - wait_preemptive", client.token.process_id());
    client
        .wait_preemptive(client.token)
        .await
        .map_err(|e| ClientError::RpcError(e.to_string()))
}

#[tracing::instrument(level = "info", skip(client, requirements, timeout))]
async fn wait_allocation(
    client: &Client,
    requirements: TaskRequirements,
    timeout: std::time::Duration,
) -> Result<ResourceAlloc, ClientError> {
    let call_res = async {
        loop {
            match client
                .wait_allocation(client.token, requirements.clone())
                .await
            {
                Ok(Ok(dev)) => {
                    if let Some(alloc) = dev {
                        info!(
                            "Client: {} - got allocation {:?}",
                            client.token.process_id(),
                            alloc
                        );
                        return Ok(alloc);
                    } else {
                        // There are not available resources at this point so we have to try
                        // again.
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                        warn!(
                            "No available resources for client: {} - waiting",
                            client.token.process_id()
                        );
                        continue;
                    }
                }
                Ok(Err(e)) => {
                    error!("{}", e.to_string());
                    return Err(e);
                }
                Err(e) => {
                    error!("{}", e.to_string());
                    return Err(ClientError::RpcError(e.to_string()));
                }
            }
        }
    };

    tokio::time::timeout(timeout, call_res)
        .await
        .map_err(|_| ClientError::Timeout)?
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
    let jrpc_client = Client::new(&server_address(), Default::default())?;
    let rt = Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?;
    rt.block_on(async { jrpc_client.list_allocations().await })
        .map_err(|e| ClientError::Other(e.to_string()))
}

#[tracing::instrument(level = "info", skip(client))]
async fn release_preemptive(client: &Client) -> Result<(), ClientError> {
    info!(
        "Release preemptive for client {}",
        client.token.process_id()
    );
    client
        .release_preemptive(client.token)
        .await
        .map_err(|e| ClientError::RpcError(e.to_string()))?
}

#[tracing::instrument(level = "info", skip(client))]
async fn release(client: &Client) -> Result<(), ClientError> {
    info!("Client: {} releasing resources", client.token.process_id());
    client
        .release(client.token)
        .await
        .map_err(|e| ClientError::RpcError(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    struct TaskTest;

    impl TaskFunc for TaskTest {
        type TaskOutput = String;

        fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::TaskOutput, Box<dyn Error>> {
            Ok("HelloWorld".to_string())
        }

        fn task(&mut self, _alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Box<dyn Error>> {
            Ok(TaskResult::Done)
        }
    }

    fn task_requirements() -> TaskRequirements {
        let start = chrono::Utc::now();
        let end = start + chrono::Duration::seconds(30);
        let deadline = Deadline::new(start, end);
        TaskReqBuilder::new()
            .resource_req(ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
                quantity: 1,
                preemptible: true,
            })
            .with_time_estimations(Duration::from_millis(500), 1, Duration::from_millis(3000))
            .with_deadline(deadline)
            .build()
            .unwrap()
    }

    #[test]
    fn calls_scheduler_one_process() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let pid: u32 = rng.gen();
        let client_id: u64 = rng.gen();
        let token = register(pid, client_id).unwrap();

        let handle = scheduler::spawn_scheduler_with_handler(&server_address()).unwrap();

        let res = schedule_one_of(
            token,
            &mut TaskTest,
            Some(task_requirements()),
            Default::default(),
        );
        // Accept just this type of error
        if let Err(e) = res {
            assert_eq!(e, ClientError::Timeout);
        }
        handle.close();
    }

    #[test]
    fn release_test() {
        // This test only check communication and well formed param parsing
        let address = "127.0.0.1:7000".to_string();
        let client = Client::new(&address, Default::default()).unwrap();
        let handle = scheduler::spawn_scheduler_with_handler(&address).unwrap();
        let rt = Runtime::new().unwrap();
        let _res_req = ResourceReq {
            resource: common::ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        };

        let res = rt
            .block_on(async { client.release(client.token).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))
            .unwrap();

        assert!(res.is_ok());

        handle.close();
    }
}
