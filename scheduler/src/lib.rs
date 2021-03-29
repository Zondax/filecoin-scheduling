mod error;
mod handler;
mod requests;
mod scheduler;
mod server;
mod solver;
mod solvers;

pub use error::Error;
pub use handler::Handler;
pub use server::RpcMethods;
pub use server::Server;
pub use solver::{ResourceState, Solver, TaskState};
use std::net::SocketAddr;

use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;

pub const STATE_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.scheduler_state");

/// Starts a json-rpc server listening to *addr*
#[tracing::instrument(level = "info")]
pub fn run_scheduler(address: &str) -> Result<(), Error> {
    let handler = scheduler::Scheduler::new(STATE_FILE_PATH);
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
pub fn spawn_scheduler_with_handler(address: &str) -> Result<CloseHandle, Error> {
    let handler = scheduler::Scheduler::new(STATE_FILE_PATH);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = address.parse().map_err(|_| Error::InvalidAddress)?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&address)
        .map_err(|e| Error::ConnectionError(e.to_string()))?;
    let close_handle = server.close_handle();

    std::thread::spawn(|| {
        server.wait();
    });

    Ok(close_handle)
}
