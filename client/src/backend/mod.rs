mod http_client;
use once_cell::sync::OnceCell;

pub use http_client::RpcCaller;
use tokio_02 as tokio;

use tokio::runtime::Runtime;
fn get_runtime() -> &'static Runtime {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Error creating tokio runtime"))
}

use crate::Error;
use scheduler::{ClientToken, DeviceId, Pid, PreemptionResponse, ResourceAlloc, TaskRequirements};

pub trait RpcCall {
    fn wait_preemptive(&self, token: &ClientToken) -> Result<PreemptionResponse, Error>;
    fn service_status(&self) -> Result<Pid, Error>;
    fn list_allocations(&self) -> Result<Vec<(DeviceId, u64)>, Error>;
    fn wait_allocation(
        &self,
        token: &ClientToken,
        task: &TaskRequirements,
    ) -> Result<Option<ResourceAlloc>, Error>;
    fn release(&self, token: &ClientToken) -> Result<(), Error>;
    fn release_preemptive(&self, token: &ClientToken) -> Result<(), Error>;
}

pub fn client_backend(address: &str) -> Result<RpcCaller, Error> {
    http_client::RpcCaller::new(address)
}
