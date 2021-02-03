use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Error {
    GlobalMutexError(String),
    RpcError(String),
    ResourceReqEmpty,
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
            Error::Other(ref descripcion) => write!(f, "Error: {}", descripcion),
        }
    }
}

impl std::error::Error for Error {}
