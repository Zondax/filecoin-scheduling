use tracing::warn;

mod client;
mod config;
mod db;
mod device;
mod error;
mod handler;
mod monitor;
mod requests;
mod requirements;
mod resource;
mod scheduler;
mod server;
mod solver;
mod solvers;

pub use crate::config::Settings;
pub use crate::scheduler::Scheduler;
pub use client::{ClientToken, Pid};
pub use db::Database;
pub use device::*;
pub use error::Error;
pub use handler::Handler;
pub use monitor::*;
pub use requests::{PreemptionResponse, RequestMethod};
pub use requirements::*;
pub use resource::*;
pub use server::RpcMethods;
pub use server::Server;
pub use solver::{ResourceState, Solver, TaskState};
use std::net::SocketAddr;

use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;
use std::path::Path;

use crossbeam::channel::bounded;

pub type Result<T> = std::result::Result<T, Error>;

/// Starts a json-rpc server listening to *addr*
#[tracing::instrument(level = "debug", skip(devices, settings, database_path))]
pub fn run_scheduler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<()> {
    let maintenance_interval = settings.service.maintenance_interval;
    let (shutdown_tx, shutdown_rx) = bounded(0);
    let db = Database::open(database_path, false)?;
    let handler = scheduler::Scheduler::new(settings.clone(), devices, Some(shutdown_tx), db)?;
    let server = Server::new(handler);
    if let Some(tick) = maintenance_interval {
        server.start_maintenance_thread(tick);
    }

    let close_handle = spawn_service(server, settings)?;

    let _ = shutdown_rx.recv().unwrap();
    close_handle.close();
    warn!("Service closed");
    Ok(())
}

#[tracing::instrument(level = "debug", skip(devices, settings, database_path))]
// To be use for testing purposes
pub fn spawn_scheduler_with_handler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<CloseHandle> {
    let db = Database::open(database_path, true)?;
    let handler = scheduler::Scheduler::new(settings.clone(), devices, None, db)?;
    let server = Server::new(handler);

    spawn_service(server, settings)
}

fn spawn_service<H: Handler>(server: Server<H>, settings: Settings) -> Result<CloseHandle> {
    let address: SocketAddr = settings
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

    Ok(close_handle)
}
