use std::io::Error as IoError;

use jsonrpc_core_client::RpcError;
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
    #[error("ConnectionError: `{0}`")]
    ConnectionError(String),
    #[error("ConfigError: `{0}`")]
    ConfigError(String),
    #[error("Unknown error: `{0}`")]
    Other(String),
}

impl From<RpcError> for Error {
    fn from(err: RpcError) -> Self {
        match &err {
            RpcError::Other(ref e) if e.to_string().contains("tcp connect error") => {
                Self::ConnectionError(e.to_string())
            }
            _ => Self::RpcError(err.to_string()),
        }
    }
}
