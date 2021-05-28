use rust_gpu_tools::opencl::DeviceUuid;

use common::{ClientToken, PreemptionResponse, ResourceAlloc, TaskRequirements};
use scheduler::Error;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn wait_allocation(
        &self,
        client: ClientToken,
        task: TaskRequirements,
    ) -> Result<Option<ResourceAlloc>, Error>;

    async fn wait_preemptive(&self, client: ClientToken) -> Result<PreemptionResponse, Error>;

    async fn check_server(&self) -> Result<(), Error>;

    async fn list_allocations(&self) -> Result<Vec<(DeviceUuid, u64)>, Error>;

    async fn release(&self, client: ClientToken) -> Result<(), Error>;

    async fn release_preemptive(&self, client: ClientToken) -> Result<(), Error>;
}

#[jsonrpc_client::implement(RpcClient)]
pub struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
    pub token: ClientToken,
}

impl Client {
    /// Creates a client
    /// `base_url` must be an address like: ip:port
    pub fn new(base_url: &str, token: ClientToken) -> Result<Self, crate::Error> {
        let address = format!("http://{}", base_url);
        let base_url = address
            .parse::<jsonrpc_client::Url>()
            .map_err(|_| crate::Error::InvalidAddress)?;
        let inner = reqwest::Client::new();
        Ok(Self {
            inner,
            base_url,
            token,
        })
    }
}
