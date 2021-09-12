use std::sync::Arc;

use futures::channel::oneshot;
use futures::FutureExt;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::{BoxFuture, Result as RpcResult};

use crate::handler::Handler;
use crate::monitor::MonitorInfo;
use crate::requests::{SchedulerRequest, SchedulerResponse};

use crate::{
    ClientToken, DeviceId, Pid, PreemptionResponse, RequestMethod, ResourceAlloc, TaskRequirements,
};

use crate::Result;
use tracing::warn;

pub type AsyncRpcResult<T> = BoxFuture<RpcResult<Result<T>>>;

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "wait_allocation")]
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
        job_context: String,
    ) -> AsyncRpcResult<Option<ResourceAlloc>>;

    #[rpc(name = "wait_preemptive")]
    fn wait_preemptive(&self, task: ClientToken) -> AsyncRpcResult<PreemptionResponse>;

    #[rpc(name = "list_allocations")]
    fn list_allocations(&self) -> AsyncRpcResult<Vec<(DeviceId, u64)>>;

    #[rpc(name = "service_status")]
    fn health_check(&self) -> BoxFuture<RpcResult<Pid>>;

    #[rpc(name = "release")]
    fn release(&self, client: ClientToken) -> AsyncRpcResult<()>;

    #[rpc(name = "release_preemptive")]
    fn release_preemptive(&self, client: ClientToken) -> AsyncRpcResult<()>;

    #[rpc(name = "abort")]
    fn abort(&self, client: Vec<Pid>) -> AsyncRpcResult<()>;

    #[rpc(name = "remove_stalled")]
    fn remove_stalled(&self, client: Vec<Pid>) -> AsyncRpcResult<()>;

    #[rpc(name = "monitoring")]
    fn monitoring(&self) -> BoxFuture<RpcResult<std::result::Result<MonitorInfo, String>>>;
}

pub struct Server<H: Handler>(Arc<H>);

impl<H> Server<H>
where
    H: Handler,
{
    pub fn new(handler: H) -> Self {
        let handler = Arc::new(handler);
        Self(handler)
    }

    pub fn start_maintenance_thread(&self, tick_interval: u64) {
        use crossbeam::channel::{select, tick};
        use std::time::Duration;
        let handler = self.0.clone();
        let ticker = tick(Duration::from_millis(tick_interval));
        std::thread::spawn(move || loop {
            select! {
                recv(ticker) -> _ => {
                    if !handler.maintenance() {
                        warn!("Closing maintenance thread");
                        break;
                    }
                },
            }
        });
    }
}

impl<H: Handler> RpcMethods for Server<H> {
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
        job_context: String,
    ) -> AsyncRpcResult<Option<ResourceAlloc>> {
        let method = RequestMethod::Schedule(client, requirements, job_context);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::Schedule(res)) => Ok(res),
            _ => unreachable!(),
        }))
    }

    fn wait_preemptive(&self, client: ClientToken) -> AsyncRpcResult<PreemptionResponse> {
        let method = RequestMethod::WaitPreemptive(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::SchedulerWaitPreemptive(res)) => Ok(res),
            _ => unreachable!(),
        }))
    }

    fn list_allocations(&self) -> AsyncRpcResult<Vec<(DeviceId, u64)>> {
        let method = RequestMethod::ListAllocations;
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::ListAllocations(res)) => Ok(res),
            _ => unreachable!(),
        }))
    }

    fn release(&self, client: ClientToken) -> AsyncRpcResult<()> {
        let method = RequestMethod::Release(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::Release) => Ok(Ok(())),
            _ => unreachable!(),
        }))
    }

    fn release_preemptive(&self, client: ClientToken) -> AsyncRpcResult<()> {
        let method = RequestMethod::ReleasePreemptive(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::ReleasePreemptive) => Ok(Ok(())),
            _ => unreachable!(),
        }))
    }

    fn abort(&self, client: Vec<Pid>) -> AsyncRpcResult<()> {
        let method = RequestMethod::Abort(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::Abort(res)) => Ok(res),
            _ => unreachable!(),
        }))
    }

    fn remove_stalled(&self, client: Vec<Pid>) -> AsyncRpcResult<()> {
        let method = RequestMethod::RemoveStalled(client);
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::RemoveStalled(res)) => Ok(res),
            _ => unreachable!(),
        }))
    }

    fn monitoring(&self) -> BoxFuture<RpcResult<std::result::Result<MonitorInfo, String>>> {
        let method = RequestMethod::Monitoring;
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::Monitoring(info)) => Ok(info),
            _ => unreachable!(),
        }))
    }

    // Endpoint for clients to check if the server instance is running
    fn health_check(&self) -> BoxFuture<RpcResult<u64>> {
        let method = RequestMethod::CheckService;
        let (sender, receiver) = oneshot::channel();
        let request = SchedulerRequest { sender, method };
        self.0.process_request(request);
        Box::pin(receiver.map(|e| match e {
            Ok(SchedulerResponse::CheckService(pid)) => Ok(pid),
            _ => unreachable!(),
        }))
    }
}
