use common::Error;
use common::ResourceAlloc;
use common::TaskRequirements;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn schedule_one_of(&self, task: TaskRequirements) -> Result<ResourceAlloc, Error>;

    async fn schedule_preemptive(&self, task: String) -> Result<String, String>;
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
