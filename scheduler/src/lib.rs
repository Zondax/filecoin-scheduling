mod config;
mod error;
mod handler;
mod monitor;
mod requests;
mod scheduler;
mod server;
mod solver;
mod solvers;

use crate::config::Settings;
pub use error::Error;
pub use handler::Handler;
pub use monitor::*;
pub use server::RpcMethods;
pub use server::Server;
pub use solver::{ResourceState, Solver, TaskState};
use std::net::SocketAddr;

use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;

// check if defining this as an ev variable is more convinient
const SETTINGS_PATH: &str = "/tmp/scheduler.toml";

/// Starts a json-rpc server listening to *addr*
#[tracing::instrument(level = "info")]
pub fn run_scheduler(address: &str, devices: common::Devices) -> Result<(), Error> {
    let settings = Settings::new(SETTINGS_PATH).map_err(|e| {
        tracing::error!("Error reading config file: {}", e.to_string());
        Error::InvalidConfig(e.to_string())
    })?;
    let handler = scheduler::Scheduler::new(settings, devices);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = address.parse().map_err(|_| Error::InvalidAddress)?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&address)
        .map_err(|e| Error::ConnectionError(e.to_string()))?;

    server.wait();
    Ok(())
}

#[tracing::instrument(level = "info")]
// To be use for testing purposes
pub fn spawn_scheduler_with_handler(
    address: &str,
    devices: common::Devices,
) -> Result<CloseHandle, Error> {
    let settings = Settings::new(SETTINGS_PATH).map_err(|e| {
        tracing::error!("Error reading config file: {}", e.to_string());
        Error::InvalidConfig(e.to_string())
    })?;
    let handler = scheduler::Scheduler::new(settings, devices);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = address.parse().map_err(|_| Error::InvalidAddress)?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&address)
        .map_err(|e| Error::ConnectionError(e.to_string()))?;
    let close_handle = server.close_handle();

    std::thread::spawn(move || {
        server.wait();
    });

    Ok(close_handle)
}
