use jsonrpc_core_client::{RpcChannel, RpcResult, TypedClient};
use jsonrpc_core_client::transports::http::connect;
use rust_gpu_tools::opencl::GPUSelector;

use common::{ClientToken, PreemptionResponse, ResourceAlloc, TaskRequirements};
use scheduler::Error;

#[derive(Debug)]
pub struct Client {
    pub base_url: String,
    pub token: ClientToken,
    /// Helper string that gives more context in logs messages
    pub context: Option<String>,
}

struct RpcHandler(TypedClient);

pub struct RpcCaller {
    handler: RpcHandler,
    pub inner: Client,
}

impl From<RpcChannel> for RpcHandler {
    fn from(channel: RpcChannel) -> Self {
        RpcHandler(channel.into())
    }
}

impl Client {
    //noinspection HttpUrlsUsage
    /// Creates a client
    /// `base_url` must be an address like: ip:port
    pub fn new(
        base_url: &str,
        token: ClientToken,
        context: Option<String>,
    ) -> Result<Self, crate::Error> {
        let address = format!("http://{}", base_url);
        Ok(Self {
            base_url: address,
            token,
            context,
        })
    }

    pub async fn connect(self) -> RpcResult<RpcCaller> {
        let inner = connect(self.base_url.as_str()).await?;
        let handler = RpcHandler(inner);
        Ok(RpcCaller {
            handler,
            inner: self,
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
                (self.inner.token,),
            )
            .await
    }

    pub async fn check_server(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method("check_server", "Result<(), Error>", ())
            .await
    }

    pub async fn list_allocations(&self) -> RpcResult<Result<Vec<(GPUSelector, u64)>, Error>> {
        self.handler
            .0
            .call_method(
                "list_allocations",
                "Result<Vec<(GPUSelector, u64)>, Error>",
                (),
            )
            .await
    }

    pub async fn wait_allocation(
        &self,
        task: TaskRequirements,
        job_context: Option<String>,
    ) -> RpcResult<Result<Option<ResourceAlloc>, Error>> {
        self.handler
            .0
            .call_method(
                "wait_allocation",
                "Result<Option<ResourceAlloc>, Error>>",
                (self.inner.token, task, job_context),
            )
            .await
    }

    pub async fn release(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method("release", "Result<(), Error>>", (self.inner.token,))
            .await
    }

    pub async fn release_preemptive(&self) -> RpcResult<Result<(), Error>> {
        self.handler
            .0
            .call_method(
                "release_preemptive",
                "Result<(), Error>>",
                (self.inner.token,),
            )
            .await
    }
}
