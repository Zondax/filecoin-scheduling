use std::io::Error as IoError;

use scheduler::Error as SchedulerError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("GlobalMutex error: `{0}`")]
    GlobalMutex(#[from] IoError),
    #[error("Rpc client error: `{0}`")]
    RpcError(String),
    #[error("Timeout")]
    Timeout,
    #[error("Scheduler error: `{0}`")]
    Scheduler(#[from] SchedulerError),
    #[error("Can not parse the provided address")]
    InvalidAddress,
    #[error("Job was aborted by an external client")]
    Aborted,
    #[error("No GPU resources on the system")]
    NoGpuResources,
    #[error("Unexpected panic in task function")]
    TaskFunctionPanics,
    #[error("Unknown error: `{0}`")]
    Other(String),
}
