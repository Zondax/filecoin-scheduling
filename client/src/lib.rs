use std::collections::HashMap;
use std::time::Duration;

use tracing::{debug, error, trace, warn};

pub use common::{
    list_devices, ClientToken, Deadline, DeviceId, Devices, Pid, PreemptionResponse, ResourceAlloc,
    ResourceMemory, ResourceReq, ResourceType, TaskEstimations, TaskFunc, TaskReqBuilder,
    TaskRequirements, TaskResult, TaskType,
};
pub use error::Error;
pub use rpc_client::{Client, RpcCaller};
use scheduler::run_scheduler;
pub use scheduler::{spawn_scheduler_with_handler, Error as SchedulerError};

pub mod error;
mod global_mutex;
mod rpc_client;

const SERVER_ADDRESS: &str = "127.0.0.1:5000";
// delay in milliseconds between calls to wait_allocation/preemptive
// this might be part of a configuration file.
const WAIT_ALLOCATION_DELAY: u64 = 500;
const WAIT_PREEMPTIVE_DELAY: u64 = 500;

// number of tries before returning an error when starting the scheduler service
const START_SERVER_RETRIES: u64 = 3;
// amount of time to wait between retries in milliseconds
const START_SERVER_DELAY: u64 = 500;

// The initial idea for testing addresses was using std::net::TcpListener::bind(x.x.x.x:0)
// that returns a random port that is not being used, but considering that we may have multiple
// processes running on tests, having a static address is the best approach so far.
const TEST_SERVER_ADDRESS: &str = "127.0.0.1:8000";

// deadline values for winning and window post tasks.
// for simplicity it is defined here but later it can move to
// other place where it makes more sense to be.
// 20 secs giving a margin of 5 secs
const WINNING_POST_END_DEADLINE: u64 = 20;
// 25min margin of 5min
const WINDOW_POST_END_DEADLINE: u64 = 1500;

// for winning post this timeout(seconds) is used to fallback to CPU
const WINNING_POST_TIMEOUT: u64 = 10;

// this function might be removed later as this
// setting is part of the configuration file
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
    unimplemented!();
}

#[tracing::instrument(level = "info", skip(context))]
pub fn register<E: From<Error>>(
    client_name: Option<String>,
    context: Option<String>,
) -> Result<Client, E> {
    let pid = palaver::thread::gettid();
    let token = ClientToken {
        pid,
        name: client_name.unwrap_or_else(|| "".to_string()),
    };
    // TODO: Here we look for the config file and get the address from there as other params as
    // well
    Client::new(
        &server_address(),
        token,
        context.unwrap_or_else(Default::default),
    )
    .map_err(E::from)
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
/// would be executed immediately without the intervention of the scheduler service. Otherwise the
/// task is scheduled on the resource that best fit the requirements. The task execution
/// will be controlled by the scheduler service.
/// * `timeout` - Indicates how much the client is able to wait for the task to be scheduled. It is
/// possible that the client have to wait for resources to be freed when other task are done. If it expires and Error would be returned indicating it was
/// the case.
#[tracing::instrument(level = "info", skip(timeout, task_func, req, client), fields(pid = client.token.pid))]
pub fn schedule_one_of<T, E: From<Error>>(
    client: Client,
    task_func: &mut dyn TaskFunc<Output = T, Error = E>,
    mut req: TaskRequirements,
    timeout: Duration,
) -> Result<T, E> {
    let address = server_address();

    let timeout = match req.task_type {
        Some(TaskType::WindowPost) => {
            // modify the deadline only if it is empty
            req.deadline
                .get_or_insert(Deadline::from_secs(0, WINDOW_POST_END_DEADLINE));
            timeout
        }
        Some(TaskType::WinningPost) => {
            // modify the deadline only if it is empty
            req.deadline
                .get_or_insert(Deadline::from_secs(0, WINNING_POST_END_DEADLINE));
            Duration::from_secs(WINNING_POST_TIMEOUT)
        }
        _ => timeout,
    };

    check_scheduler_service_or_launch(address)?;
    let caller = client.connect()?;
    let allocation = wait_allocation(&caller, req, timeout)?;
    let result = execute_task(&caller, timeout, task_func, &allocation);
    let _ = release(&caller);
    result
}

pub fn execute_without_scheduler<T, E>(
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
fn execute_task<'a, T, E: From<Error>>(
    client: &RpcCaller,
    timeout: Duration,
    task: &mut dyn TaskFunc<Output = T, Error = E>,
    alloc: &ResourceAlloc,
) -> Result<T, E> {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    task.init(Some(alloc))?;
    loop {
        let preemptive_state = wait_preemptive(client, timeout)?;

        match preemptive_state {
            PreemptionResponse::Wait => {}
            PreemptionResponse::Execute => {
                trace!(
                    "client: {}:{} from: {} - Calling task function",
                    client.inner.token.pid,
                    client.inner.token.name,
                    client.inner.context,
                );
                // try to handle possible panics
                let result = catch_unwind(AssertUnwindSafe(|| task.task(Some(alloc))));
                if let Err(_error) = result {
                    let _ = release_preemptive(client);
                    let _ = release(client);
                    error!(
                        "Client: {}:{} in {} panics",
                        client.inner.token.pid, client.inner.token.name, client.inner.context,
                    );
                    // TODO: Look for ways to show the panic message. without propagating the panic
                    return Err(E::from(Error::TaskFunctionPanics));
                }
                let cont = result.unwrap()?;
                trace!("Client {} task iteration completed", client.inner.token.pid);
                release_preemptive(client)?;
                if cont == TaskResult::Done {
                    break;
                }
            }
            PreemptionResponse::Abort => {
                warn!(
                    "Client: {}:{} from: {} - aborted",
                    client.inner.token.pid, client.inner.token.name, client.inner.context
                );
                return Err(E::from(Error::Aborted));
            }
        }
    }

    task.end(Some(alloc))
}

#[tracing::instrument(level = "info", skip(client, requirements, timeout), fields(pid = client.inner.token.pid))]
fn wait_allocation(
    client: &RpcCaller,
    requirements: TaskRequirements,
    timeout: std::time::Duration,
) -> Result<ResourceAlloc, Error> {
    use std::time::Instant;
    let start = Instant::now();
    loop {
        let alloc_state = client
            .wait_allocation(requirements.clone(), client.inner.context.clone())
            .map_err(|e| Error::RpcError(e.to_string()))??;
        if let Some(alloc) = alloc_state {
            debug!(
                "Client: {}:{} from: {} - got allocation {:?}",
                client.inner.token.pid,
                client.inner.token.name,
                client.inner.context,
                alloc.devices,
            );
            return Ok(alloc);
        }
        if start.elapsed() > timeout {
            return Err(Error::Timeout);
        }
        std::thread::sleep(Duration::from_millis(WAIT_ALLOCATION_DELAY));
        // There are not available resources at this point so we have to try
        // again.
        warn!(
            "Client: {} - Resources not available - waiting",
            client.inner.token.pid
        );
    }
}

#[tracing::instrument(level = "info", skip(client, timeout), fields(pid = client.inner.token.pid))]
fn wait_preemptive(client: &RpcCaller, timeout: Duration) -> Result<PreemptionResponse, Error> {
    use std::time::Instant;
    let start = Instant::now();
    loop {
        let response = client
            .wait_preemptive()
            .map_err(|e| Error::RpcError(e.to_string()))?
            .map_err(Error::Scheduler);
        if let Ok(PreemptionResponse::Wait) = response {
            if start.elapsed() > timeout {
                return Err(Error::Timeout);
            }
            std::thread::sleep(Duration::from_millis(WAIT_PREEMPTIVE_DELAY));
        } else {
            return response;
        }
    }
}

#[tracing::instrument(level = "info", skip(client), fields(pid = client.inner.token.pid))]
fn release_preemptive(client: &RpcCaller) -> Result<(), Error> {
    client
        .release_preemptive()
        .map_err(|e| Error::RpcError(e.to_string()))?
        .map_err(Error::Scheduler)
}

#[tracing::instrument(level = "info", skip(client), fields(pid = client.inner.token.pid))]
fn release(client: &RpcCaller) -> Result<(), Error> {
    client
        .release()
        .map_err(|e| Error::RpcError(e.to_string()))?
        .map_err(Error::Scheduler)
}

#[allow(dead_code)]
#[tracing::instrument(level = "debug", skip(address))]
fn launch_scheduler_process(address: String) -> Result<(), Error> {
    use global_mutex::GlobalMutex;
    use nix::unistd::{fork, ForkResult};

    // check if there are resources to manage before trying to start the scheduler service
    let devices = common::list_devices();
    if devices.gpu_devices().is_empty() {
        return Err(Error::NoGpuResources);
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            // number of retries to check that the scheduler-service is running
            let mut retries = START_SERVER_RETRIES;
            std::thread::sleep(Duration::from_millis(START_SERVER_DELAY));
            while let Err(e) = check_scheduler_service(address.clone()) {
                // make the parent process wait for the service to run
                warn!("service has not been started yet, trying again in 500 ms");
                retries -= 1;
                if retries == 0 {
                    return Err(Error::Other(format!(
                        "Can not start scheduler service: {}",
                        e.to_string()
                    )));
                }
                std::thread::sleep(Duration::from_millis(START_SERVER_DELAY));
            }
            Ok(())
        }
        Ok(ForkResult::Child) => match GlobalMutex::try_lock() {
            Ok(guard) => {
                let mut retries = START_SERVER_RETRIES;
                while let Err(e) = run_scheduler(&address, devices.clone()) {
                    error!(err = %e,"Got error trying to start the server");
                    retries -= 1;
                    if retries == 0 {
                        return Err(Error::Other(format!(
                            "Can not start scheduler service: {}",
                            e.to_string()
                        )));
                    }
                }
                drop(guard);
                Ok(())
            }
            Err(e) => {
                error!(err = %e,"Error acquiring lock");
                debug!("another process started the scheduler - exiting");
                Err(e)
            }
        },
        Err(e) => Err(Error::Other(e.to_string())),
    }
}

/// Helper function for creating a ResourceReq list
/// - Get the current allocations in the scheduler, push any resource that has not been allocated and use it as requirements
/// - If there are not available resources, which means all memory is used
/// it would list the raw devices information and use that as requirements.
pub fn resources_as_requirements() -> Result<Vec<common::ResourceReq>, Error> {
    // Get the current devices state.
    // removing those that do not have available memory
    let mut resources = list_allocations()?;
    resources.retain(|_, memory| *memory > 0);

    // Push the devices that has no been allocated
    // or in case there are not available. Just get the current devices in the system and propose
    // them as a requirement
    common::list_devices().gpu_devices().iter().for_each(|dev| {
        let selector = dev.device_id();
        resources.entry(selector).or_insert_with(|| dev.memory());
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
pub fn list_allocations() -> Result<HashMap<DeviceId, u64>, Error> {
    check_scheduler_service_or_launch(server_address())?;
    let client = Client::new(&server_address(), Default::default(), Default::default())?;
    let client = client.connect()?;
    let res = client
        .list_allocations()
        .map_err(|e| {
            error!(err = %e, "Got error listing allocations: ");
            Error::Other(e.to_string())
        })
        .map(|res| res.unwrap());
    res.map(|vec| vec.into_iter().collect::<HashMap<DeviceId, u64>>())
}

#[tracing::instrument(level = "debug")]
fn check_scheduler_service_or_launch(address: String) -> Result<(), Error> {
    if check_scheduler_service(address.clone()).is_ok() {
        Ok(())
    } else {
        warn!("Scheduler service not running - trying to launch it");
        launch_scheduler_process(address)
    }
}

#[tracing::instrument(level = "debug")]
fn check_scheduler_service(address: String) -> Result<Pid, Error> {
    let client = Client::new(&address, Default::default(), Default::default())?;
    let client = client.connect()?;
    match client.check_server() {
        Ok(pid) => {
            debug!("Scheduler service running, PID: {}", pid);
            Ok(pid)
        }
        Err(e) => {
            let err = e.to_string();
            error!(err = %e, "Scheduler service not running");
            Err(Error::Other(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TaskTest;

    struct TaskTestPanic;

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

    impl TaskFunc for TaskTestPanic {
        type Output = String;
        type Error = Error;

        fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error> {
            Ok("HelloWorld".to_string())
        }

        fn task(&mut self, _alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error> {
            panic!();
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
            .with_time_estimations(Duration::from_millis(500), 1)
            .with_deadline(Some(deadline))
            .build()
    }

    #[test]
    fn calls_scheduler_one_process() {
        let token = register::<Error>(None, None).unwrap();

        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&server_address(), devices).unwrap();

        let res = schedule_one_of(
            token,
            &mut TaskTest,
            task_requirements(),
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
        let address = server_address();
        let client = Client::new(&address, Default::default(), Default::default()).unwrap();
        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&address, devices).unwrap();
        let _res_req = ResourceReq {
            resource: common::ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        };

        let client = client.connect().unwrap();
        let res = client.release();
        handle.close();

        assert!(res.is_ok());
    }

    #[test]
    fn test_panic_handler() {
        let address = server_address();
        let client = Client::new(&address, Default::default(), Default::default()).unwrap();
        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&address, devices).unwrap();
        let _res_req = ResourceReq {
            resource: common::ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        };

        let res = schedule_one_of(
            client,
            &mut TaskTestPanic,
            task_requirements(),
            Duration::from_secs(60),
        );
        handle.close();

        assert!(matches!(res.unwrap_err(), Error::TaskFunctionPanics));
    }
}
