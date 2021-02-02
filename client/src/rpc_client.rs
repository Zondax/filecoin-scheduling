use crate::error::ClientError;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn schedule(&self, task: String) -> Result<String, String>;

    async fn schedule_preemptive(&self, task: String) -> Result<String, String>;
}

#[jsonrpc_client::implement(RpcClient)]
pub struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
}

impl Client {
    pub fn new(base_url: &str) -> std::result::Result<Self, ClientError> {
        let base_url = base_url
            .parse::<jsonrpc_client::Url>()
            .map_err(|e| ClientError::RpcError(e.to_string()))?;
        let inner = reqwest::Client::new();
        Ok(Self { inner, base_url })
    }
}
