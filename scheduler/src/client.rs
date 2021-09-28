use crate::Pid;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
pub struct ClientToken {
    pub pid: Pid,
    pub name: String,
    pub context: String,
}

impl Default for ClientToken {
    fn default() -> Self {
        let pid = palaver::thread::gettid();
        ClientToken {
            pid,
            name: String::new(),
            context: String::new(),
        }
    }
}
