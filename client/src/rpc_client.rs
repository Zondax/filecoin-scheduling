use jsonrpc_core_client::transports::http::connect;
use jsonrpc_core_client::{RpcChannel, TypedClient};
use tokio::runtime::Runtime;

use super::Error as ClientError;
use common::{ClientToken, DeviceId, Pid, PreemptionResponse, ResourceAlloc, TaskRequirements};
use scheduler::Error;

use once_cell::sync::OnceCell;

fn get_runtime() -> &'static Runtime {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Error creating tokio runtime"))
}

#[derive(Debug, Clone)]
pub struct Client {
    pub base_url: String,
    pub token: ClientToken,
    /// Helper string that gives more context in logs messages
    /// if it is not set a None value is the default
    pub context: String,
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
    /// Creates a client
    /// `base_url` must be an address like: ip:port
    pub fn new(base_url: &str, token: ClientToken, context: String) -> Result<Self, crate::Error> {
        let address = format!("http://{}", base_url);
        Ok(Self {
            base_url: address,
            token,
            context,
        })
    }

    pub fn connect(self) -> Result<RpcCaller, ClientError> {
        let handle = get_runtime().handle();
        let inner = handle.block_on(async { connect(self.base_url.as_str()).await })?;
        let handler = RpcHandler(inner);
        Ok(RpcCaller {
            handler,
            inner: self,
        })
    }
}
impl RpcCaller {
    pub fn wait_preemptive(&self) -> Result<PreemptionResponse, ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<PreemptionResponse, Error>>(
                        "wait_preemptive",
                        "Result<PreemptionResponse, Error>",
                        (self.inner.token.clone(),),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn check_server(&self) -> Result<Pid, ClientError> {
        let handle = get_runtime().handle();
        Ok(handle.block_on(async {
            self.handler
                .0
                .call_method("service_status", "Pid", ())
                .await
        })?)
    }

    pub fn list_allocations(&self) -> Result<Vec<(DeviceId, u64)>, ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<Vec<(DeviceId, u64)>, Error>>(
                        "list_allocations",
                        "Result<Vec<(DeviceId, u64)>, Error>",
                        (),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn wait_allocation(
        &self,
        task: TaskRequirements,
        job_context: String,
    ) -> Result<Option<ResourceAlloc>, ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<Option<ResourceAlloc>, Error>>(
                        "wait_allocation",
                        "Result<Option<ResourceAlloc>, Error>>",
                        (self.inner.token.clone(), task, job_context),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn release(&self) -> Result<(), ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<(), Error>>(
                        "release",
                        "Result<(), Error>>",
                        (self.inner.token.clone(),),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn release_preemptive(&self) -> Result<(), ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<(), Error>>(
                        "release_preemptive",
                        "Result<(), Error>>",
                        (self.inner.token.clone(),),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }
}
