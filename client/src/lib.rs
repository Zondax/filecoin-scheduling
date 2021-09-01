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
use scheduler::run_scheduler;
pub use scheduler::{spawn_scheduler_with_handler, Error as SchedulerError, Settings};
use std::path::PathBuf;

pub mod error;
mod global_mutex;
mod rpc_client;

// delay in milliseconds between calls to wait_allocation/preemptive
// this might be part of a configuration file.
const WAIT_ALLOCATION_DELAY: u64 = 500;
const WAIT_PREEMPTIVE_DELAY: u64 = 500;

// number of tries before returning an error when starting the scheduler service
const START_SERVER_RETRIES: u64 = 3;
// amount of time to wait between retries in milliseconds
const START_SERVER_DELAY: u64 = 500;

const SCHEDULER_DB_NAME: &str = "scheduler_db";
const SCHEDULER_CONFIG_NAME: &str = "scheduler.config.toml";

#[cfg(not(dummy_devices))]
pub fn get_config_path() -> Result<PathBuf, Error> {
    let path = if let Ok(val) = std::env::var("SCHEDULER_CONFIG_PATH") {
        let path: PathBuf = val.into();
        path
    } else {
        let mut path =
            dirs::config_dir().ok_or_else(|| Error::Other("Unsupported platform".to_string()))?;
        path.push("filecoin/");
        path
    };
    // check that the dirs exist otherwise create them if possible
    if !path.is_dir() {
        std::fs::create_dir_all(&path).map_err(|e| {
            Error::ConfigError(format!("cannot create config dir {}", e.to_string()))
        })?;
    }
    Ok(path)
}

#[cfg(dummy_devices)]
pub fn get_config_path() -> Result<PathBuf, Error> {
    Ok(PathBuf::new().join("/tmp/"))
}

#[derive(Clone)]
pub struct Client {
    pub token: ClientToken,
    pub context: String,
    pub(crate) rpc_caller: RpcCaller,
    settings: Settings,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("token", &self.token)
            .field("context", &self.context)
            .field("rpc_caller", &"http")
            .field("settings", &self.settings)
            .finish()
    }
}

impl Client {
    #[tracing::instrument(level = "info")]
    pub fn register<E: From<Error>>() -> Result<Client, E> {
        let mut path = get_config_path()?;
        path.push(SCHEDULER_CONFIG_NAME);
        let settings = Settings::new(path).map_err(|e| Error::ConfigError(e.to_string()))?;
        Client::new(Default::default(), settings).map_err(E::from)
    }

    pub fn register_with_settings<E: From<Error>>(settings: Settings) -> Result<Client, E> {
        Client::new(Default::default(), settings).map_err(E::from)
    }

    /// Creates a client
    /// `address` must be an address like: ip:port
    fn new(token: ClientToken, settings: Settings) -> Result<Self, crate::Error> {
        let base_url = format!("http://{}", settings.service.address);
        let rpc_caller = RpcCaller::new(base_url.as_str())?;
        let client = Self {
            token,
            context: String::new(),
            rpc_caller,
            settings,
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
    /// possible that the client have to wait for resources to be freed when other task are done. If it expires an error would be returned indicating it was
    /// the case.
    #[tracing::instrument(level = "info", skip(self,timeout, task_func, req), fields(pid = self.token.pid))]
    pub fn schedule_one_of<T, E: From<Error>>(
        &self,
        task_func: &mut dyn TaskFunc<Output = T, Error = E>,
        req: TaskRequirements,
        timeout: Duration,
    ) -> Result<T, E> {
        let timeout = req
            .task_type
            .and_then(|t| {
                self.settings
                    .tasks_settings
                    .iter()
                    .find(|s| s.task_type == t)
                    .map(|task| Duration::from_secs(task.timeout))
            })
            .unwrap_or(timeout);

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

    /// Returns a tuple with the ID and available memory of devices being used
    pub fn list_allocations(&self) -> Result<HashMap<DeviceId, u64>, Error> {
        let res = self.rpc_caller.list_allocations()?;

        Ok(res.into_iter().collect::<HashMap<DeviceId, u64>>())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn check_scheduler_service_or_launch(&self) -> Result<(), Error> {
        if self.check_scheduler_service().is_ok() {
            Ok(())
        } else {
            self.launch_scheduler_process()
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
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
                    let mut path = get_config_path()?;
                    path.push(SCHEDULER_DB_NAME);
                    while let Err(e) =
                        run_scheduler(self.settings.clone(), path.clone(), devices.clone())
                    {
                        retries -= 1;
                        if retries == 0 {
                            error!(err = %e, "Failed starting scheduler service");
                            return Err(Error::Scheduler(e));
                        }
                    }
                    drop(guard);
                    Ok(())
                }
                Err(e) => {
                    debug!("another process started the scheduler - exiting");
                    error!(err = %e,"Error acquiring lock");
                    Err(e)
                }
            },
            Err(e) => Err(Error::Other(e.to_string())),
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
        let mut settings = Settings::new(SCHEDULER_CONFIG_NAME).unwrap();
        settings.service.address = "127.0.0.1:8000".to_string();
        let client = Client::register_with_settings::<Error>(settings.clone()).unwrap();

        let devices = common::list_devices();
        let handle = spawn_scheduler_with_handler(settings, "/tmp/one_task/", devices).unwrap();

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
        let mut settings = Settings::new(SCHEDULER_CONFIG_NAME).unwrap();
        settings.service.address = "127.0.0.1:10000".to_string();
        let client = Client::register_with_settings::<Error>(settings.clone()).unwrap();
        let devices = common::list_devices();
        let handle = spawn_scheduler_with_handler(settings, "/tmp/release/", devices).unwrap();

        let res = client.release();
        handle.close();

        assert!(res.is_ok());
    }

    #[test]
    fn test_panic_handler() {
        let mut settings = Settings::new(SCHEDULER_CONFIG_NAME).unwrap();
        settings.service.address = "127.0.0.1:9000".to_string();
        let client = Client::register_with_settings::<Error>(settings.clone()).unwrap();
        let devices = common::list_devices();
        let handle =
            spawn_scheduler_with_handler(settings, "/tmp/panic_handler/", devices).unwrap();

        let res = client.schedule_one_of(
            &mut TaskTestPanic,
            task_requirements(),
            Duration::from_secs(60),
        );
        handle.close();

        assert!(matches!(res.unwrap_err(), Error::TaskFunctionPanics));
    }
}
