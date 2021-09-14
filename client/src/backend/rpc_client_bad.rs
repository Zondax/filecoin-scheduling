#![cfg(feature = "rpc_client")]

use super::get_runtime;
use super::Error as ClientError;
use scheduler::{
    ClientToken, DeviceId, Error, Pid, PreemptionResponse, ResourceAlloc, TaskRequirements,
};

use super::RpcCall;

#[jsonrpc_client::api]
pub trait RpcClient {
    async fn wait_preemptive(&self, token: &ClientToken) -> Result<PreemptionResponse, Error>;
    async fn service_status(&self) -> Pid;
    async fn list_allocations(&self) -> Result<Vec<(DeviceId, u64)>, Error>;
    async fn wait_allocation(
        &self,
        token: &ClientToken,
        task: &TaskRequirements,
    ) -> Result<Option<ResourceAlloc>, Error>;
    async fn release(&self, token: &ClientToken) -> Result<(), Error>;
    async fn release_preemptive(&self, token: &ClientToken) -> Result<(), Error>;
}

#[jsonrpc_client::implement(RpcClient)]
#[derive(Clone, Debug)]
pub struct Caller {
    base_url: jsonrpc_client::Url,
    inner: reqwest::Client,
}

#[derive(Clone)]
pub struct RpcCaller {
    handler: Caller,
}

impl RpcCaller {
    pub fn new(base_url: &str) -> Result<RpcCaller, ClientError> {
        let inner = reqwest::Client::new();
        let base_url = base_url
            .parse::<jsonrpc_client::Url>()
            .map_err(|e| ClientError::Other(e.to_string()))?;
        let handler = Caller { inner, base_url };
        Ok(RpcCaller { handler })
    }
}

impl RpcCall for RpcCaller {
    fn wait_preemptive(&self, token: &ClientToken) -> Result<PreemptionResponse, ClientError> {
        get_runtime()
            .block_on(async { self.handler.wait_preemptive(token).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?
            .map_err(ClientError::Scheduler)
    }

    fn service_status(&self) -> Result<u64, ClientError> {
        Ok(get_runtime()
            .block_on(async { self.handler.service_status().await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?)
    }

    fn list_allocations(&self) -> Result<Vec<(DeviceId, u64)>, ClientError> {
        get_runtime()
            .block_on(async { self.handler.list_allocations().await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?
            .map_err(ClientError::Scheduler)
    }

    fn wait_allocation(
        &self,
        token: &ClientToken,
        task: &TaskRequirements,
    ) -> Result<Option<ResourceAlloc>, ClientError> {
        get_runtime()
            .block_on(async { self.handler.wait_allocation(token, task).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?
            .map_err(ClientError::Scheduler)
    }

    fn release(&self, token: &ClientToken) -> Result<(), ClientError> {
        get_runtime()
            .block_on(async { self.handler.release(token).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?
            .map_err(ClientError::Scheduler)
    }

    fn release_preemptive(&self, token: &ClientToken) -> Result<(), ClientError> {
        get_runtime()
            .block_on(async { self.handler.release_preemptive(token).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?
            .map_err(ClientError::Scheduler)
    }
}
