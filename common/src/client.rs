#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ClientToken {
    pub pid: u32,
    pub client_id: u64,
}
