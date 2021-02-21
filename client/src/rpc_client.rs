use common::{ClientToken, Error, ResourceAlloc, TaskRequirements};

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn wait_allocation(
        &self,
        client: ClientToken,
        task: TaskRequirements,
    ) -> Result<Option<ResourceAlloc>, Error>;

    async fn wait_preemptive(&self, task: crate::ClientToken, t: std::time::Duration) -> bool;

    async fn check_server(&self) -> Result<(), Error>;

    async fn list_allocations(&self) -> Vec<u32>;

    async fn release(&self, alloc: ResourceAlloc) -> Result<(), Error>;
}

#[jsonrpc_client::implement(RpcClient)]
pub struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
}

impl Client {
    /// Creates a json_rpc client
    /// `base_url` must be an address like: ip:port
    pub fn new(base_url: &str) -> std::result::Result<Self, Error> {
        let address = format!("http://{}", base_url);
        let base_url = address
            .parse::<jsonrpc_client::Url>()
            .map_err(|e| Error::RpcError(e.to_string()))?;
        let inner = reqwest::Client::new();
        Ok(Self { inner, base_url })
    }
}
