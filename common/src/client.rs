pub type Pid = u64;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
pub struct ClientToken {
    pub pid: Pid,
    pub name: String,
}
