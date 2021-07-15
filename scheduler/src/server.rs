use std::result::Result;

use futures::channel::oneshot;
use futures::FutureExt;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::{BoxFuture, Result as RpcResult};
use rust_gpu_tools::opencl::GPUSelector;

use crate::handler::Handler;
use crate::monitor::MonitorInfo;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::Error;
use common::{
    ClientToken, PreemptionResponse, RequestMethod, ResourceAlloc, TaskId, TaskRequirements,
};

type AllocationResult = Result<Vec<(GPUSelector, u64)>, Error>;
pub type AsyncRpcResult<T> = BoxFuture<RpcResult<Result<T, Error>>>;

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "wait_allocation")]
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
        job_context: Option<String>,
    ) -> AsyncRpcResult<Option<ResourceAlloc>>;

    #[rpc(name = "wait_preemptive")]
    fn wait_preemptive(
        &self,
        task: ClientToken,
    ) -> BoxFuture<RpcResult<Result<PreemptionResponse, Error>>>;

    #[rpc(name = "list_allocations")]
    fn list_allocations(&self) -> BoxFuture<RpcResult<AllocationResult>>;

    #[rpc(name = "check_server")]
    fn health_check(&self) -> BoxFuture<RpcResult<Result<(), Error>>>;

    #[rpc(name = "release")]
    fn release(&self, client: ClientToken) -> BoxFuture<RpcResult<Result<(), Error>>>;

    #[rpc(name = "release_preemptive")]
    fn release_preemptive(&self, client: ClientToken) -> BoxFuture<RpcResult<Result<(), Error>>>;

    #[rpc(name = "abort")]
    fn abort(&self, client: Vec<TaskId>) -> BoxFuture<RpcResult<Result<(), Error>>>;

    #[rpc(name = "remove_stalled")]
    fn remove_stalled(&self, client: Vec<TaskId>) -> BoxFuture<RpcResult<Result<(), Error>>>;

    #[rpc(name = "monitoring")]
    fn monitoring(&self) -> BoxFuture<RpcResult<Result<MonitorInfo, String>>>;
}

pub struct Server<H: Handler>(H);

impl<H> Server<H>
where
    H: Handler,
{
    pub fn new(handler: H) -> Self {
        Self(handler)
    }
}

impl<H: Handler> RpcMethods for Server<H> {
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
        job_context: Option<String>,
    ) -> AsyncRpcResult<Option<ResourceAlloc>> {
        let method = RequestMethod::Schedule(client, requirements, job_context);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::Schedule(res)) => Ok(res),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn wait_preemptive(
        &self,
        client: ClientToken,
    ) -> BoxFuture<RpcResult<Result<PreemptionResponse, Error>>> {
        let method = RequestMethod::WaitPreemptive(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::SchedulerWaitPreemptive(res)) => Ok(res),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn list_allocations(&self) -> BoxFuture<RpcResult<AllocationResult>> {
        let method = RequestMethod::ListAllocations;
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::ListAllocations(res)) => Ok(res),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn release(&self, client: ClientToken) -> BoxFuture<RpcResult<Result<(), Error>>> {
        let method = RequestMethod::Release(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::Release) => Ok(Ok(())),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn release_preemptive(&self, client: ClientToken) -> BoxFuture<RpcResult<Result<(), Error>>> {
        let method = RequestMethod::ReleasePreemptive(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::ReleasePreemptive) => Ok(Ok(())),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn abort(&self, client: Vec<TaskId>) -> BoxFuture<RpcResult<Result<(), Error>>> {
        let method = RequestMethod::Abort(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::Abort(res)) => Ok(res),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn remove_stalled(&self, client: Vec<TaskId>) -> BoxFuture<RpcResult<Result<(), Error>>> {
        let method = RequestMethod::RemoveStalled(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::RemoveStalled(res)) => Ok(res),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    fn monitoring(&self) -> BoxFuture<RpcResult<Result<MonitorInfo, String>>> {
        let method = RequestMethod::Monitoring;
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::Monitoring(info)) => Ok(info),
                    _ => unreachable!(),
                })
                .boxed(),
        )
    }

    // Endpoint for clients to check if the server instance is running
    fn health_check(&self) -> BoxFuture<RpcResult<Result<(), Error>>> {
        Box::pin(async { Ok(Ok(())) })
    }
}
