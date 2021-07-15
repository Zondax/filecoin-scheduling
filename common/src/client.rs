pub type TaskId = u64;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ClientToken {
    pub pid: TaskId,
    pub name: String,
}
