use jsonrpc_core_client::{RpcChannel, TypedClient};
use tokio::runtime::Runtime;

use super::Error as ClientError;
use scheduler::{
    ClientToken, DeviceId, Error, Pid, PreemptionResponse, ResourceAlloc, TaskRequirements,
};

use once_cell::sync::OnceCell;

fn get_runtime() -> &'static Runtime {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Error creating tokio runtime"))
}

#[derive(Clone)]
struct RpcHandler(TypedClient);

#[derive(Clone)]
pub struct RpcCaller {
    handler: RpcHandler,
}

impl From<RpcChannel> for RpcHandler {
    fn from(channel: RpcChannel) -> Self {
        RpcHandler(channel.into())
    }
}
impl RpcCaller {
    pub fn new(base_url: &str) -> Result<RpcCaller, ClientError> {
        use jsonrpc_core_client::transports::http::connect;

        let handle = get_runtime().handle();
        let inner = handle.block_on(async { connect(base_url).await })?;
        let handler = RpcHandler(inner);
        Ok(RpcCaller { handler })
    }

    pub fn wait_preemptive(&self, token: &ClientToken) -> Result<PreemptionResponse, ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<PreemptionResponse, Error>>(
                        "wait_preemptive",
                        "Result<PreemptionResponse, Error>",
                        (token,),
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
        token: &ClientToken,
        task: &TaskRequirements,
        job_context: &str,
    ) -> Result<Option<ResourceAlloc>, ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<Option<ResourceAlloc>, Error>>(
                        "wait_allocation",
                        "Result<Option<ResourceAlloc>, Error>>",
                        (token, task, job_context),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn release(&self, token: &ClientToken) -> Result<(), ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<(), Error>>("release", "Result<(), Error>>", (token,))
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }

    pub fn release_preemptive(&self, token: &ClientToken) -> Result<(), ClientError> {
        get_runtime()
            .handle()
            .block_on(async {
                self.handler
                    .0
                    .call_method::<_, Result<(), Error>>(
                        "release_preemptive",
                        "Result<(), Error>>",
                        (token,),
                    )
                    .await
            })?
            .map_err(ClientError::Scheduler)
    }
}
