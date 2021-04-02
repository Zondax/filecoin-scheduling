#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("Invalid address format")]
    InvalidAddress,
    #[error("Connection error `{0}`")]
    ConnectionError(String),
    #[error("Error: `{0}`")]
    Other(String),
    #[error("Resource requirements list is empty")]
    ResourceReqEmpty,
    #[error("Can not read/write scheduler state - try later")]
    RwError,
    #[error("Error creating solver")]
    NoSolver,
    #[error("Solver error: `{0}`")]
    SolverOther(String),
}
