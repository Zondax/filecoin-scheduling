use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

use futures::channel::oneshot;
use futures::FutureExt;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use common::{ClientToken, Error, RequestMethod, ResourceAlloc, TaskRequirements};

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "wait_allocation")]
    fn wait_allocation(
        &self,
        requirements: TaskRequirements,
    ) -> BoxFuture<Result<std::result::Result<ResourceAlloc, Error>>>;

    #[rpc(name = "schedule_preemptive")]
    fn preemptive(&self, task: String) -> BoxFuture<Result<String>>;

    #[rpc(name = "list_allocations")]
    fn list_allocations(&self) -> BoxFuture<Result<Vec<u32>>>;

    #[rpc(name = "wait_preemptive")]
    fn wait_preemptive(&self, task: ClientToken, t: std::time::Duration)
        -> BoxFuture<Result<bool>>;

    #[rpc(name = "check_server")]
    fn health_check(&self) -> BoxFuture<Result<()>>;
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
        requirements: TaskRequirements,
    ) -> BoxFuture<Result<std::result::Result<ResourceAlloc, Error>>> {
        let method = RequestMethod::Schedule(requirements);
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

    fn preemptive(&self, task: String) -> BoxFuture<Result<String>> {
        let method = RequestMethod::SchedulePreemptive(task);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::SchedulePreemptive(res)) => Ok(res),
                    _ => Ok("Preemptive".to_string()),
                })
                .boxed(),
        )
    }

    fn list_allocations(&self) -> BoxFuture<Result<Vec<u32>>> {
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

    fn wait_preemptive(
        &self,
        client: ClientToken,
        t: std::time::Duration,
    ) -> BoxFuture<Result<bool>> {
        let method = RequestMethod::WaitPreemptive(client, t);
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

    // Endpoint for clients to check if the server instance is running
    fn health_check(&self) -> BoxFuture<Result<()>> {
        Box::pin(async { Ok(()) })
    }
}
