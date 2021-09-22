use crossbeam::channel::{bounded, select, tick, Receiver};
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::channel::oneshot;
use futures::FutureExt;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::{BoxFuture, Result as RpcResult};

use crate::handler::Handler;
use crate::monitor::MonitorInfo;
use crate::requests::{SchedulerRequest, SchedulerResponse};

use crate::{
    ClientToken, DeviceId, Error, Pid, PreemptionResponse, RequestMethod, ResourceAlloc, Settings,
    TaskRequirements,
};

use super::{CloseService, Service};
use crate::Result;
use tracing::warn;

struct CloseServiceImpl(Option<CloseHandle>);

impl CloseService for CloseServiceImpl {
    fn close_service(&mut self) -> Result<()> {
        // take the handle as we can not call close()
        // twice
        self.0
            .take()
            .map(|h| {
                h.close();
            })
            .ok_or_else(|| Error::Other("Service not running or already closed".to_string()))
    }
}

#[derive(Default)]
pub struct HttpService {
    settings: Settings,
}

impl HttpService {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

pub type AsyncRpcResult<T> = BoxFuture<RpcResult<Result<T>>>;

#[rpc(server)]
pub trait RpcMethods {
    #[rpc(name = "wait_allocation")]
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
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

    pub fn start_maintenance_thread(&self, tick_interval: u64) -> Receiver<()> {
        use std::time::Duration;
        let (shutdown_tx, shutdown_rx) = bounded(0);
        let handler = self.0.clone();
        let ticker = tick(Duration::from_millis(tick_interval));
        std::thread::spawn(move || loop {
            let should_continue = handler.maintenance();
            select! {
                recv(ticker) -> _ => {
                    if !should_continue {
                        shutdown_tx.send(()).unwrap();
                        warn!("Closing maintenance thread");
                        break;
                    }
                },
            }
        });
        shutdown_rx
    }
}

impl<H: Handler> Service<H> for HttpService {
    fn start_service(&self, handler: H) -> Result<()> {
        let server = Server::new(handler);
        let maintenance_interval = self.settings.service.maintenance_interval;

        let address: SocketAddr = self
            .settings
            .service
            .address
            .parse()
            .map_err(|_| Error::InvalidAddress)?;
        let mut shutdown_rx =
            maintenance_interval.map(|tick| server.start_maintenance_thread(tick));

        let mut io = IoHandler::new();
        io.extend_with(server.to_delegate());

        let server = ServerBuilder::new(io)
            .threads(num_cpus::get())
            .start_http(&address)
            .map_err(|e| {
                // Close the maintenance thread
                shutdown_rx.take();
                Error::ConnectionError(e.to_string())
            })?;
        let close_handle = server.close_handle();
        if let Some(rx) = shutdown_rx {
            std::thread::spawn(move || {
                server.wait();
            });
            rx.recv().unwrap();
            warn!("Closed!!!!!!!!!!!!!!!!!!!!");
            close_handle.close();
        } else {
            warn!("no tick");
            server.wait();
        }

        Ok(())
    }

    fn spawn_service(&self, handler: H) -> Result<Box<dyn super::CloseService>> {
        let server = Server::new(handler);

        let address: SocketAddr = self
            .settings
            .service
            .address
            .parse()
            .map_err(|_| Error::InvalidAddress)?;

        let mut io = IoHandler::new();
        io.extend_with(server.to_delegate());

        let server = ServerBuilder::new(io)
            .threads(num_cpus::get())
            .start_http(&address)
            .map_err(|e| Error::ConnectionError(e.to_string()))?;
        let close_handle = server.close_handle();
        std::thread::spawn(move || {
            server.wait();
        });
        Ok(Box::new(CloseServiceImpl(Some(close_handle))))
    }
}

impl<H: Handler> RpcMethods for Server<H> {
    fn wait_allocation(
        &self,
        client: ClientToken,
        requirements: TaskRequirements,
    ) -> AsyncRpcResult<Option<ResourceAlloc>> {
        let method = RequestMethod::Schedule(client, requirements);
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
