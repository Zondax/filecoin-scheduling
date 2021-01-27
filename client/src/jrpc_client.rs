use crate::error::ClientError;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn schedule(&self, task: String) -> Result<String, ClientError>;

    async fn schedule_preemptive(&self, task: String) -> Result<String, ClientError>;
}

#[jsonrpc_client::implement(RpcClient)]
pub(crate) struct Client {
    inner: reqwest::Client,
    base_url: jsonrpc_client::Url,
}

impl Client {
    pub(crate) fn new(base_url: &str) -> Result<Self, ClientError> {
        let base_url = base_url
            .parse::<jsonrpc_client::Url>()
            .map_err(|e| ClientError::Other(e.to_string()))?;
        let inner = reqwest::Client::new();
        Ok(Self { inner, base_url })
    }
}
