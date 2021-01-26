mod handler;
mod requests;
mod scheduler;
mod server;

pub use requests::{RequestMethod, SchedulerResponse};
pub use server::RpcMethods;

use std::error::Error;
use std::net::SocketAddr;

use jsonrpc_core::IoHandler;

use jsonrpc_http_server::{CloseHandle, ServerBuilder};

use common::SERVER_ADDRESS;

const STATE_FILE_PATH: &'static str = concat!(env!("CARGO_TARGET_DIR"), "scheduler_state");

/// Starts a json-rpc server listening to *addr*
pub fn run_scheduler() -> Result<CloseHandle, Box<dyn Error>> {
    let handler = scheduler::Scheduler::new(STATE_FILE_PATH);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = SERVER_ADDRESS.parse()?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io).start_http(&address)?;
    let close_handle = server.close_handle();

    std::thread::spawn(|| {
        server.wait();
    });

    Ok(close_handle)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
