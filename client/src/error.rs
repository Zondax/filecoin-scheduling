use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClientError {
    GlobalMutexError(String),
    RpcError(String),
    Other(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::GlobalMutexError(ref descripcion) => {
                write!(f, "Global Mutex error: {}", descripcion)
            }
            ClientError::RpcError(ref e) => write!(f, "Rpc error: {}", e),
            ClientError::Other(ref descripcion) => write!(f, "Error: {}", descripcion),
        }
    }
}

impl Error for ClientError {}
