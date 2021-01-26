use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

use futures::channel::oneshot;
use futures::FutureExt;

use crate::handler::Handler;
use crate::requests::{RequestMethod, SchedulerRequest, SchedulerResponse};

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "schedule")]
    fn schedule(&self, task: String) -> BoxFuture<Result<String>>;

    #[rpc(name = "schedule_preemptive")]
    fn preemptive(&self, task: String) -> BoxFuture<Result<String>>;
}

pub(crate) struct Server<H: Handler>(H);

impl<H> Server<H>
where
    H: Handler,
{
    pub fn new(handler: H) -> Self {
        Self(handler)
    }
}

impl<H: Handler> RpcMethods for Server<H> {
    fn schedule(&self, task: String) -> BoxFuture<Result<String>> {
        let method = RequestMethod::Schedule(task);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(
            receiver
                .map(|e| match e {
                    Ok(SchedulerResponse::Schedule(res)) => Ok(res),
                    _ => Ok("Preemptive".to_string()),
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
