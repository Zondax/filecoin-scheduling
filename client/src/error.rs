use scheduler::Error as SchedulerError;
use std::io::Error as IoError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("GlobalMutex error")]
    GlobalMutex(#[from] IoError),
    #[error("Rpc client error ")]
    RpcError(#[from] jsonrpc_client::Error<reqwest::Error>),
    #[error("Timeout")]
    Timeout,
    #[error("Scheduler error")]
    Scheduler(#[from] SchedulerError),
    #[error("Can not parse the provided address")]
    InvalidAddress,
    #[error("Unknown error: `{0}`")]
    Other(String),
}
