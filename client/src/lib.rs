use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use tracing::{debug, error, trace, warn};

pub use common::{
    list_devices, ClientToken, Deadline, DeviceId, Devices, Pid, PreemptionResponse, ResourceAlloc,
    ResourceMemory, ResourceReq, ResourceType, TaskEstimations, TaskFunc, TaskReqBuilder,
    TaskRequirements, TaskResult, TaskType,
};
pub use error::Error;
pub use rpc_client::RpcCaller;
use scheduler::{run_scheduler, Settings};
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

#[derive(Clone)]
pub struct Client {
    pub address: String,
    pub token: ClientToken,
    /// Helper string that gives more context in logs messages
    /// if it is not set a None value is the default
    pub context: String,
    pub(crate) rpc_caller: RpcCaller,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("adress", &self.address)
            .field("token", &self.token)
            .field("context", &self.context)
            .field("rpc_caller", &"http")
            .finish()
    }
}

impl Client {
    #[tracing::instrument(level = "info")]
    pub fn register<E: From<Error>>() -> Result<Client, E> {
        let pid = palaver::thread::gettid();
        let token = ClientToken {
            pid,
            name: String::new(),
        };
        // TODO: Here we look for the config file and get the address from there as other params as
        // well
        Client::new(&server_address(), token).map_err(E::from)
    }
    /// Creates a client
    /// `address` must be an address like: ip:port
    fn new(address: &str, token: ClientToken) -> Result<Self, crate::Error> {
        let base_url = format!("http://{}", address);
        let rpc_caller = RpcCaller::new(&base_url.as_str())?;
        let client = Self {
            address: address.to_owned(),
            token,
            context: String::new(),
            rpc_caller,
        };
        // start service if it is not running already
        client.check_scheduler_service_or_launch()?;
        Ok(client)
    }

    pub fn set_name<T: ToString>(&mut self, name: T) {
        self.token.name = name.to_string();
    }

    pub fn set_context<T: ToString>(&mut self, context: T) {
        self.context = context.to_string();
    }
}

impl Client {
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
    #[tracing::instrument(level = "info", skip(self,timeout, task_func, req), fields(pid = self.token.pid))]
    pub fn schedule_one_of<T, E: From<Error>>(
        &self,
        task_func: &mut dyn TaskFunc<Output = T, Error = E>,
        mut req: TaskRequirements,
        timeout: Duration,
    ) -> Result<T, E> {
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

        //self.check_scheduler_service_or_launch(address)?;
        let allocation = self.wait_allocation(req, timeout)?;
        let result = self.execute_task(timeout, task_func, &allocation);
        let _ = self.release();
        result
    }

    pub fn execute_without_scheduler<T, E>(
        &self,
        task_func: &mut dyn TaskFunc<Output = T, Error = E>,
    ) -> Result<T, E> {
        task_func.init(None)?;
        let mut cont = TaskResult::Continue;
        while cont == TaskResult::Continue {
            cont = task_func.task(None)?;
        }
        task_func.end(None)
    }

    #[tracing::instrument(level = "info", skip(self, timeout, task, alloc))]
    fn execute_task<'a, T, E: From<Error>>(
        &self,
        timeout: Duration,
        task: &mut dyn TaskFunc<Output = T, Error = E>,
        alloc: &ResourceAlloc,
    ) -> Result<T, E> {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        task.init(Some(alloc))?;
        loop {
            let preemptive_state = self.wait_preemptive(timeout)?;

            match preemptive_state {
                PreemptionResponse::Wait => {}
                PreemptionResponse::Execute => {
                    trace!(
                        "client: {}:{} from: {} - Calling task function",
                        self.token.pid,
                        self.token.name,
                        self.context,
                    );
                    // try to handle possible panics
                    let result = catch_unwind(AssertUnwindSafe(|| task.task(Some(alloc))));
                    if let Err(_error) = result {
                        let _ = self.release_preemptive();
                        let _ = self.release();
                        error!(
                            "Client: {}:{} in {} panics",
                            self.token.pid, self.token.name, self.context,
                        );
                        // TODO: Look for ways to show the panic message. without propagating the panic
                        return Err(E::from(Error::TaskFunctionPanics));
                    }
                    let cont = result.unwrap()?;
                    trace!("Client {} task iteration completed", self.token.pid);
                    self.release_preemptive()?;
                    if cont == TaskResult::Done {
                        break;
                    }
                }
                PreemptionResponse::Abort => {
                    warn!(
                        "Client: {}:{} from: {} - aborted",
                        self.token.pid, self.token.name, self.context,
                    );
                    return Err(E::from(Error::Aborted));
                }
            }
        }

        task.end(Some(alloc))
    }

    #[tracing::instrument(level = "info", skip(self, requirements, timeout), fields(pid = self.token.pid))]
    fn wait_allocation(
        &self,
        requirements: TaskRequirements,
        timeout: std::time::Duration,
    ) -> Result<ResourceAlloc, Error> {
        use std::time::Instant;
        let start = Instant::now();
        loop {
            let alloc_state =
                self.rpc_caller
                    .wait_allocation(&self.token, &requirements, &self.context)?;
            if let Some(alloc) = alloc_state {
                debug!(
                    "Client: {}:{} from: {} - got allocation {:?}",
                    self.token.pid, self.token.name, self.context, alloc.devices,
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
                self.token.pid
            );
        }
    }

    #[tracing::instrument(level = "info", skip(self, timeout), fields(pid = self.token.pid))]
    fn wait_preemptive(&self, timeout: Duration) -> Result<PreemptionResponse, Error> {
        use std::time::Instant;
        let start = Instant::now();
        loop {
            let response = self.rpc_caller.wait_preemptive(&self.token);
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

    #[tracing::instrument(level = "info", skip(self), fields(pid = self.token.pid))]
    fn release_preemptive(&self) -> Result<(), Error> {
        self.rpc_caller.release_preemptive(&self.token)
    }

    #[tracing::instrument(level = "info", skip(self), fields(pid = self.token.pid))]
    fn release(&self) -> Result<(), Error> {
        self.rpc_caller.release(&self.token)
    }

    /// Helper function for creating a ResourceReq list
    /// - Get the current allocations in the scheduler, push any resource that has not been allocated and use it as requirements
    /// - If there are not available resources, which means all memory is used
    /// it would list the raw devices information and use that as requirements.
    //pub fn resources_as_requirements() -> Result<Vec<common::ResourceReq>, Error> {
    //// Get the current devices state.
    //// removing those that do not have available memory
    //let mut resources = list_allocations()?;
    //resources.retain(|_, memory| *memory > 0);

    //// Push the devices that has no been allocated
    //// or in case there are not available. Just get the current devices in the system and propose
    //// them as a requirement
    //common::list_devices().gpu_devices().iter().for_each(|dev| {
    //let selector = dev.device_id();
    //resources.entry(selector).or_insert_with(|| dev.memory());
    //});

    //// map to memory => quantity
    //let mut reqs: HashMap<u64, usize> = HashMap::new();
    //resources.into_iter().for_each(|(_, memory)| {
    //let entry = reqs.entry(memory).or_insert(0);
    //*entry += 1;
    //});
    //Ok(reqs
    //.into_iter()
    //.map(|(memory, quantity)| ResourceReq {
    //resource: ResourceType::Gpu(ResourceMemory::Mem(memory)),
    //quantity,
    //preemptible: true, // by default the resource is preemptible assuming the task will perform more than 1 iteration
    //})
    //.collect::<Vec<_>>())
    //}

    /// Returns a tuple with the ID and available memory of devices being used
    pub fn list_allocations(&self) -> Result<HashMap<DeviceId, u64>, Error> {
        let res = self.rpc_caller.list_allocations()?;

        Ok(res.into_iter().collect::<HashMap<DeviceId, u64>>())
    }

    #[tracing::instrument(level = "debug")]
    fn check_scheduler_service_or_launch(&self) -> Result<(), Error> {
        if self.check_scheduler_service().is_ok() {
            Ok(())
        } else {
            println!("Starting service");
            self.launch_scheduler_process()
        }
    }

    #[tracing::instrument(level = "debug")]
    fn check_scheduler_service(&self) -> Result<Pid, Error> {
        let pid = self.rpc_caller.check_server()?;
        debug!("Scheduler service running, PID: {}", pid);
        Ok(pid)
    }

    #[allow(dead_code)]
    #[tracing::instrument(level = "debug", skip(self))]
    fn launch_scheduler_process(&self) -> Result<(), Error> {
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
                while let Err(e) = self.check_scheduler_service() {
                    retries -= 1;
                    if retries == 0 {
                        error!(err = %e, "Failed starting scheduler service");
                        return Err(e);
                    }
                    std::thread::sleep(Duration::from_millis(START_SERVER_DELAY));
                }
                Ok(())
            }
            Ok(ForkResult::Child) => match GlobalMutex::try_lock() {
                Ok(guard) => {
                    let mut retries = START_SERVER_RETRIES;
                    while let Err(e) = run_scheduler(&self.address, devices.clone()) {
                        retries -= 1;
                        println!("RETRIES {}", retries);
                        if retries == 0 {
                            error!(err = %e, "Failed starting scheduler service");
                            return Err(Error::Scheduler(e));
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
}

#[tracing::instrument(level = "debug")]
fn check_scheduler_service(address: String) -> Result<Pid, Error> {
    let client = Client::register::<Error>()?;
    let pid = client.rpc_caller.check_server()?;
    debug!("Scheduler service running, PID: {}", pid);
    Ok(pid)
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
        let client = Client::register::<Error>().unwrap();

        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&server_address(), devices).unwrap();

        let res = client.schedule_one_of(&mut TaskTest, task_requirements(), Default::default());
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
        let client = Client::register::<Error>().unwrap();
        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&address, devices).unwrap();
        let _res_req = ResourceReq {
            resource: common::ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        };

        let res = client.release();
        handle.close();

        assert!(res.is_ok());
    }

    #[test]
    fn test_panic_handler() {
        let address = server_address();
        let client = Client::register::<Error>().unwrap();
        let devices = common::list_devices();
        let handle = scheduler::spawn_scheduler_with_handler(&address, devices).unwrap();
        let _res_req = ResourceReq {
            resource: common::ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        };

        let res = client.schedule_one_of(
            &mut TaskTestPanic,
            task_requirements(),
            Duration::from_secs(60),
        );
        handle.close();

        assert!(matches!(res.unwrap_err(), Error::TaskFunctionPanics));
    }
}
