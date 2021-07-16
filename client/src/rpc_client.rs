use jsonrpc_core_client::transports::http::connect;
use jsonrpc_core_client::{RpcChannel, RpcError, RpcResult, TypedClient};
use rust_gpu_tools::opencl::GPUSelector;
use tokio::runtime::Runtime;

use super::Error as ClientError;
use common::{ClientToken, PreemptionResponse, ResourceAlloc, TaskRequirements};
use scheduler::Error;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
static CELL: OnceCell<Mutex<Runtime>> = OnceCell::new();

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

    pub fn connect(self) -> Result<RpcCaller, ClientError> {
        let runtime = Mutex::new(Runtime::new().map_err(|e| ClientError::Other(e.to_string()))?);

        let _ = CELL.set(runtime);
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        let inner = runtime
            .block_on(async { connect(self.base_url.as_str()).await })
            .map_err(|e| ClientError::RpcError(e.to_string()))?;
        let handler = RpcHandler(inner);
        Ok(RpcCaller {
            handler,
            inner: self,
        })
    }
}
impl RpcCaller {
    pub fn wait_preemptive(&self) -> RpcResult<Result<PreemptionResponse, Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method(
                    "wait_preemptive",
                    "Result<PreemptionResponse, Error>",
                    (self.inner.token.clone(),),
                )
                .await
        })
    }

    pub fn check_server(&self) -> RpcResult<Result<(), Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method("check_server", "Result<(), Error>", ())
                .await
        })
    }

    pub fn list_allocations(&self) -> RpcResult<Result<Vec<(GPUSelector, u64)>, Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method(
                    "list_allocations",
                    "Result<Vec<(GPUSelector, u64)>, Error>",
                    (),
                )
                .await
        })
    }

    pub fn wait_allocation(
        &self,
        task: TaskRequirements,
        job_context: Option<String>,
    ) -> RpcResult<Result<Option<ResourceAlloc>, Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method(
                    "wait_allocation",
                    "Result<Option<ResourceAlloc>, Error>>",
                    (self.inner.token.clone(), task, job_context),
                )
                .await
        })
    }

    pub fn release(&self) -> RpcResult<Result<(), Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method("release", "Result<(), Error>>", (self.inner.token.clone(),))
                .await
        })
    }

    pub fn release_preemptive(&self) -> RpcResult<Result<(), Error>> {
        let mut runtime = CELL.get().unwrap().lock().unwrap();
        runtime.block_on(async {
            self.handler
                .0
                .call_method(
                    "release_preemptive",
                    "Result<(), Error>>",
                    (self.inner.token.clone(),),
                )
                .await
        })
    }
}
