use anyhow::Result;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn abort(&self, client: u64) -> std::result::Result<(), scheduler::Error>;
    async fn monitoring(&self) -> std::result::Result<scheduler::MonitorInfo, String>;
}

#[jsonrpc_client::implement(RpcClient)]
pub struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
}

impl Client {
    pub fn new(base_url: &str) -> Result<Self> {
        let address = format!("http://{}", base_url);
        let base_url = address.parse::<jsonrpc_client::Url>()?;
        let inner = reqwest::Client::new();
        Ok(Self { inner, base_url })
    }
}
