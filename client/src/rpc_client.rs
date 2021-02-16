use common::{Error, ResourceAlloc, TaskRequirements};

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn wait_allocation(&self, task: TaskRequirements) -> Result<ResourceAlloc, Error>;

    async fn schedule_preemptive(&self, task: String) -> Result<String, Error>;

    async fn check_server(&self) -> Result<(), Error>;

    async fn list_allocations(&self) -> Result<Vec<u32>, Error>;
}

#[jsonrpc_client::implement(RpcClient)]
pub struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
}

impl Client {
    pub fn new(base_url: &str) -> std::result::Result<Self, Error> {
        let base_url = base_url
            .parse::<jsonrpc_client::Url>()
            .map_err(|e| Error::RpcError(e.to_string()))?;
        let inner = reqwest::Client::new();
        Ok(Self { inner, base_url })
    }
}
