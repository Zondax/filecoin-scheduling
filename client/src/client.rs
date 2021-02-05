#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientToken {
    pub(crate) pid: u32,
    pub(crate) client_id: u64,
}

impl ClientToken {
    pub fn new(pid: u32, client_id: u64) -> Self {
        Self { pid, client_id }
    }
}
