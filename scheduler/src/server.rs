use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

use futures::channel::oneshot;
use futures::FutureExt;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use common::{Error, RequestMethod, ResourceAlloc, TaskRequirements};

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "schedule_one_of")]
    fn schedule_one_of(
        &self,
        requirements: TaskRequirements,
    ) -> BoxFuture<Result<std::result::Result<ResourceAlloc, Error>>>;

    #[rpc(name = "schedule_preemptive")]
    fn preemptive(&self, task: String) -> BoxFuture<Result<String>>;
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
    fn schedule_one_of(
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
}
