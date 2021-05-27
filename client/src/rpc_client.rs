//use futures::{future, Future, FutureExt, StreamExt, TryFutureExt};
use rust_gpu_tools::opencl::DeviceUuid;

use common::{ClientToken, PreemptionResponse, ResourceAlloc, TaskRequirements};
//use jsonrpc_client_transports::{RpcError, RpcResult};
use jsonrpc_core_client::transports::http::connect;
//use jsonrpc_core_client::*;
use jsonrpc_core_client::{RpcChannel, RpcResult, TypedClient};
use scheduler::Error;

pub struct Client {
    base_url: String,
    pub token: ClientToken,
}

struct RpcHandler(TypedClient);

pub struct RpcCaller {
    handler: RpcHandler,
    base_url: String,
    pub token: ClientToken,
}

impl From<RpcChannel> for RpcHandler {
    fn from(channel: RpcChannel) -> Self {
        RpcHandler(channel.into())
    }
}

impl Client {
    /// Creates a client
    /// `base_url` must be an address like: ip:port
    pub fn new(base_url: &str, token: ClientToken) -> Result<Self, crate::Error> {
        let address = format!("http://{}", base_url);
        Ok(Self {
            base_url: address,
            token,
        })
    }

    pub async fn connect(self) -> RpcResult<RpcCaller> {
        let inner = connect(self.base_url.as_str()).await?;
        let handler = RpcHandler(inner);
        Ok(RpcCaller {
            handler,
            base_url: self.base_url,
            token: self.token,
        })
    }
}

impl RpcCaller {
    pub async fn wait_preemptive(&self) -> RpcResult<Result<PreemptionResponse, Error>> {
        self.handler
            .0
            .call_method(
                "wait_preemptive",
                "Result<PreemptionResponse, Error>",
                (self.token,),
            )
            .await
    }

    pub async fn check_server(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method("check_server", "Result<(), Error>", ())
            .await
    }

    pub async fn list_allocations(&self) -> RpcResult<Result<Vec<(DeviceUuid, u64)>, Error>> {
        self.handler
            .0
            .call_method(
                "list_allocations",
                "Result<Vec<(DeviceUuid, u64)>, Error>",
                (),
            )
            .await
    }

    pub async fn wait_allocation(
        &self,
        task: TaskRequirements,
    ) -> RpcResult<Result<Option<ResourceAlloc>, Error>> {
        self.handler
            .0
            .call_method(
                "wait_allocation",
                "Result<Option<ResourceAlloc>, Error>>",
                (self.token, task),
            )
            .await
    }

    pub async fn release(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method("release", "Result<(), Error>>", (self.token,))
            .await
    }

    pub async fn release_preemptive(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method("release_preemptive", "Result<(), Error>>", (self.token,))
            .await
    }
}
