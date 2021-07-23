use tracing::error;

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
use std::path::PathBuf;

const SCHEDULER_CONFIG_NAME: &str = "scheduler.toml";

fn get_config_path() -> Result<PathBuf, Error> {
    let mut path = if let Ok(val) = std::env::var("SCHEDULER_CONFIG_PATH") {
        let path: PathBuf = val.into();
        path
    } else {
        let mut path =
            dirs::config_dir().ok_or_else(|| Error::Other("Unsuported platform".to_string()))?;
        path.push("filecoin/");
        path
    };
    // check that the dirs exist otherwise create them if possible
    if !path.is_dir() {
        std::fs::create_dir_all(&path)
            .map_err(|e| Error::Other(format!("can not create config dir {}", e.to_string())))?;
    }
    path.push(SCHEDULER_CONFIG_NAME);
    Ok(path)
}

/// Starts a json-rpc server listening to *addr*
#[tracing::instrument(level = "info")]
pub fn run_scheduler(address: &str, devices: common::Devices) -> Result<(), Error> {
    let path = get_config_path()?;
    let settings = Settings::new(path).map_err(|e| {
        error!(err = %e, "Error reading config file");
        Error::InvalidConfig(e.to_string())
    })?;
    let handler = scheduler::Scheduler::new(settings, devices);
    let server = Server::new(handler);
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
    let path = get_config_path()?;
    let settings = Settings::new(path).map_err(|e| {
        error!(err = %e, "Error reading config file");
        Error::InvalidConfig(e.to_string())
    })?;
    let handler = scheduler::Scheduler::new(settings, devices);
    let server = Server::new(handler);
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
