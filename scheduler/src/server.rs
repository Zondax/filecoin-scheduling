use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::{BoxFuture, Result};

use futures::channel::oneshot;
use futures::FutureExt;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use crate::Error;
use common::{ClientToken, RequestMethod, ResourceAlloc, TaskRequirements};

type AllocationResult = std::result::Result<Vec<(u64, u64)>, Error>;

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "wait_allocation")]
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
    ) -> BoxFuture<Result<std::result::Result<Option<ResourceAlloc>, Error>>>;

    #[rpc(name = "wait_preemptive")]
    fn wait_preemptive(&self, task: ClientToken) -> BoxFuture<Result<bool>>;

    #[rpc(name = "list_allocations")]
    fn list_allocations(&self) -> BoxFuture<Result<AllocationResult>>;

    #[rpc(name = "check_server")]
    fn health_check(&self) -> BoxFuture<Result<std::result::Result<(), Error>>>;

    #[rpc(name = "release")]
    fn release(&self, client: ClientToken) -> BoxFuture<Result<std::result::Result<(), Error>>>;

    #[rpc(name = "release_preemptive")]
    fn release_preemptive(
        &self,
        client: ClientToken,
    ) -> BoxFuture<Result<std::result::Result<(), Error>>>;
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
    ) -> BoxFuture<Result<std::result::Result<Option<ResourceAlloc>, Error>>> {
        let method = RequestMethod::Schedule(client, requirements);
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

    fn wait_preemptive(&self, client: ClientToken) -> BoxFuture<Result<bool>> {
        let method = RequestMethod::WaitPreemptive(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::SchedulerWaitPreemptive(res)) => Ok(res),
                    _ => Ok(true),
                })
                .boxed(),
        )
    }

    fn list_allocations(&self) -> BoxFuture<Result<AllocationResult>> {
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

    // For some reason we can not return () here, the is a bug on the client library that
    // expects a Result, Option or a Sized type.
    fn release(&self, client: ClientToken) -> BoxFuture<Result<std::result::Result<(), Error>>> {
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

    fn release_preemptive(
        &self,
        client: ClientToken,
    ) -> BoxFuture<Result<std::result::Result<(), Error>>> {
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

    // Endpoint for clients to check if the server instance is running

    // For some reason we can not return () here, there is a bug on the client library that
    // expects a Result, Option or a Sized type.
    fn health_check(&self) -> BoxFuture<Result<std::result::Result<(), Error>>> {
        Box::pin(async { Ok(Ok(())) })
    }
}
