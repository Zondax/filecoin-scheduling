use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info, warn};

pub mod error;
mod global_mutex;
mod rpc_client;

pub use common::{
    list_devices, ClientToken, Deadline, Devices, PreemptionResponse, ResourceAlloc,
    ResourceMemory, ResourceReq, ResourceType, TaskEstimations, TaskFunc, TaskReqBuilder,
    TaskRequirements, TaskResult, TaskType,
};
//pub use error::*;
pub use rpc_client::*;
use scheduler::run_scheduler;
pub use scheduler::spawn_scheduler_with_handler;

use tokio::runtime::Runtime;

pub use error::Error;
use rpc_client::{Client, RpcClient};

const SERVER_ADDRESS: &str = "127.0.0.1:5000";

// The initial idea for testing addresses was using std::net::TcpListener::bind(x.x.x.x:0)
// that returns a random port that is not being used, but considering that we may have multiple
// processes running on tests, having a static address is the best approach so far.
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
pub fn abort(_client: ClientToken) -> Result<(), Error> {
    Ok(())
}

#[tracing::instrument(level = "info", skip(pid, client_id))]
pub fn register<E: From<Error>>(pid: u32, client_id: u64) -> Result<Client, E> {
    info!("new client: {} - with process_id: {}", client_id, pid);
    let token = ClientToken::new(pid, client_id);
    // TODO: Here we look for the config file and get the address from there as other params as
    // well
    Client::new(&server_address(), token).map_err(E::from)
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
pub fn schedule_one_of<T, E: From<Error>>(
    client: Client,
    task_func: &mut dyn TaskFunc<Output = T, Error = E>,
    req: Option<TaskRequirements>,
    timeout: Duration,
) -> Result<T, E> {
    let address = server_address();

    if req.is_none() {
        return execute_without_scheduler(task_func);
    }

    let req = req.unwrap();

    let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;

    rt.block_on(async {
        check_scheduler_service_or_launch(address).await?;
        let allocation = wait_allocation(&client, req, timeout).await?;
        let result = execute_task(&client, timeout, task_func, &allocation).await;
        let _ = release(&client).await;
        result
    })
}

fn execute_without_scheduler<T, E>(
    task_func: &mut dyn TaskFunc<Output = T, Error = E>,
) -> Result<T, E> {
    task_func.init(None)?;
    let mut cont = TaskResult::Continue;
    while cont == TaskResult::Continue {
        cont = task_func.task(None)?;
    }
    task_func.end(None)
}

#[tracing::instrument(level = "info", skip(client, timeout, task, alloc))]
async fn execute_task<'a, T, E: From<Error>>(
    client: &Client,
    timeout: Duration,
    task: &mut dyn TaskFunc<Output = T, Error = E>,
    alloc: &ResourceAlloc,
) -> Result<T, E> {
    task.init(Some(alloc))?;
    loop {
        let preemtive_state = wait_preemptive(client, timeout).await?;
        match preemtive_state {
            PreemptionResponse::Wait => {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                continue;
            }
            PreemptionResponse::Execute => {
                let cont = task.task(Some(alloc))?;
                release_preemptive(client).await?;
                info!(
                    "Client {} task iteration completed",
                    client.token.process_id()
                );
                if cont == TaskResult::Done {
                    break;
                }
            }
            PreemptionResponse::Abort => {
                warn!("Client {} aborted", client.token.process_id());
                release_preemptive(client).await?;
                return Err(E::from(Error::Aborted));
            }
        }
    }

    task.end(Some(alloc))
}

#[tracing::instrument(level = "info", skip(client, requirements, timeout), fields(pid = client.token.pid))]
async fn wait_allocation(
    client: &Client,
    requirements: TaskRequirements,
    timeout: std::time::Duration,
) -> Result<ResourceAlloc, Error> {
    let call_res = async {
        loop {
            if let Some(alloc) = client
                .wait_allocation(client.token, requirements.clone())
                .await?
                .map_err(Error::Scheduler)?
            {
                info!(
                    "Client: {} - got allocation {:?}",
                    client.token.process_id(),
                    alloc
                );
                return Ok(alloc);
            } else {
                // There are not available resources at this point so we have to try
                // again.
                tokio::time::sleep(Duration::from_millis(500)).await;
                warn!(
                    "No available resources for client: {} - waiting",
                    client.token.process_id()
                );
                continue;
            }
        }
    };

    tokio::time::timeout(timeout, call_res)
        .await
        .map_err(|_| Error::Timeout)?
}

#[tracing::instrument(level = "info", skip(client, _timeout), fields(pid = client.token.pid))]
async fn wait_preemptive(client: &Client, _timeout: Duration) -> Result<PreemptionResponse, Error> {
    client
        .wait_preemptive(client.token)
        .await?
        .map_err(Error::Scheduler)
}

#[tracing::instrument(level = "info", skip(client), fields(pid = client.token.pid))]
async fn release_preemptive(client: &Client) -> Result<(), Error> {
    info!(
        "Release preemptive for client {}",
        client.token.process_id()
    );
    client
        .release_preemptive(client.token)
        .await?
        .map_err(Error::Scheduler)
}

#[tracing::instrument(level = "info", skip(client))]
async fn release(client: &Client) -> Result<(), Error> {
    info!("Client: {} releasing resources", client.token.process_id());
    client
        .release(client.token)
        .await?
        .map_err(Error::Scheduler)
}

#[allow(dead_code)]
#[tracing::instrument(level = "info", skip(address))]
fn launch_scheduler_process(address: String) -> Result<(), Error> {
    use global_mutex::GlobalMutex;
    use nix::unistd::{fork, ForkResult};
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            // make the parent process wait for the service to run
            info!("waiting for service to start");
            std::thread::sleep(Duration::from_millis(1000));
            Ok(())
        }
        Ok(ForkResult::Child) => {
            let mutex = GlobalMutex::new()?;
            if mutex.try_lock().is_ok() {
                run_scheduler(&address).unwrap();
            } else {
                info!("another process launching the scheduler - exiting");
            }
            Ok(())
        }
        Err(e) => Err(Error::Other(e.to_string())),
    }
}

/// Helper function for creating a ResourceReq list
/// - Get the current allocations in the scheduler, push any resource thas has not been allocated and use it as requirements
/// - If there are not available resources, which means all memory is used
/// it would list the raw devices information and use that as requirements.
pub fn resources_as_requirements() -> Result<Vec<common::ResourceReq>, Error> {
    // Get the current devices state.
    // removing those that do not have available memory
    let mut resources = list_allocations()?;
    resources.retain(|_, memory| *memory > 0);

    // Push the devices that has no been allocated
    // or in case there are not available. Just get the current devices inthe system and propose
    // them as a requirement
    common::list_devices().gpu_devices().iter().for_each(|dev| {
        resources.entry(dev.hash()).or_insert_with(|| dev.memory());
    });

    // map to memory => quantity
    let mut reqs: HashMap<u64, usize> = HashMap::new();
    resources.into_iter().for_each(|(_, memory)| {
        let entry = reqs.entry(memory).or_insert(0);
        *entry += 1;
    });
    Ok(reqs
        .into_iter()
        .map(|(memory, quantity)| ResourceReq {
            resource: ResourceType::Gpu(ResourceMemory::Mem(memory)),
            quantity,
            preemptible: true, // by default the resource is preemptible assuming the task will perform more than 1 iteration
        })
        .collect::<Vec<_>>())
}

/// Returns a tuple with the ID and available memory of devices being used
pub fn list_allocations() -> Result<HashMap<u64, u64>, Error> {
    let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
    let res = rt
        .block_on(async {
            check_scheduler_service_or_launch(server_address()).await?;
            let client = Client::new(&server_address(), Default::default())?;
            client.list_allocations().await.map_err(|e| {
                error!("{}", e.to_string());
                Error::Other(e.to_string())
            })
        })
        .map(|res| res.unwrap());
    res.map(|vec| vec.into_iter().collect::<HashMap<u64, u64>>())
}

#[tracing::instrument(level = "info")]
async fn check_scheduler_service_or_launch(address: String) -> Result<(), Error> {
    let client = Client::new(&address, Default::default())?;
    if client.check_server().await.is_ok() {
        info!("Scheduler service already started");
        Ok(())
    } else {
        warn!("Scheduler service not running - trying to launch it");
        launch_scheduler_process(address)
    }
}

pub fn get_device_by_hash(hash: u64) -> Option<common::Device> {
    let devices = common::list_devices();
    for dev in devices.gpu_devices() {
        if dev.hash() == hash {
            return Some(dev.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TaskTest;

    impl TaskFunc for TaskTest {
        type Output = String;
        type Error = Error;

        fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error> {
            Ok("HelloWorld".to_string())
        }

        fn task(&mut self, _alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error> {
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
            .with_deadline(Some(deadline))
            .build()
    }

    #[test]
    fn calls_scheduler_one_process() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let pid: u32 = rng.gen();
        let client_id: u64 = rng.gen();
        let token = register::<Error>(pid, client_id).unwrap();

        let handle = scheduler::spawn_scheduler_with_handler(&server_address()).unwrap();

        let res = schedule_one_of(
            token,
            &mut TaskTest,
            Some(task_requirements()),
            Default::default(),
        );
        // Accept just this type of error
        if let Err(e) = res {
            assert!(matches!(e, Error::Timeout));
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
            .unwrap();

        assert!(res.is_ok());

        handle.close();
    }
}
