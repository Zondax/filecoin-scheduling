use std::fmt;

//TODO: Use a more generic error type so we can propagate the error source
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Error {
    GlobalMutexError(String),
    ClientInit(String),
    ClientEnd(String),
    ClientTask(String),
    RpcError(String),
    ResourceReqEmpty,
    UnknownResource(u32),
    Timeout,
    Solver(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::GlobalMutexError(ref descripcion) => {
                write!(f, "Global Mutex error: {}", descripcion)
            }
            Error::RpcError(ref e) => write!(f, "Rpc error: {}", e),
            Error::ResourceReqEmpty => write!(f, "Requirements for task is empty"),
            Error::UnknownResource(r) => write!(f, "Resource {} not available", r),
            Error::Timeout => write!(f, "Timeout triggered before receiving a response "),
            Error::Other(ref descripcion) => write!(f, "Error: {}", descripcion),
            Error::ClientInit(ref descripcion) => {
                write!(f, "Error calling client's init function: {}", descripcion)
            }
            Error::ClientEnd(ref descripcion) => {
                write!(f, "Error calling client's end function: {}", descripcion)
            }
            Error::ClientTask(ref descripcion) => {
                write!(f, "Error calling client's task function: {}", descripcion)
            }
            Error::Solver(ref e) => {
                write!(f, "A solver error: {}", e)
            }
        }
    }
}

impl std::error::Error for Error {}
