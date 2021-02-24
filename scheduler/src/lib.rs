mod handler;
mod linearsolver;
mod requests;
mod scheduler;
mod server;

pub use handler::Handler;
pub use server::RpcMethods;
pub use server::Server;

use std::error::Error;
use std::net::SocketAddr;

use jsonrpc_core::IoHandler;

use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;

const STATE_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.scheduler_state");

/// Starts a json-rpc server listening to *addr*
pub fn run_scheduler(address: &str) -> Result<(), Box<dyn Error>> {
    let handler = scheduler::Scheduler::new(STATE_FILE_PATH);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = address.parse()?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io).start_http(&address)?;

    server.wait();
    Ok(())
}

pub fn spawn_scheduler_with_handler(address: &str) -> Result<CloseHandle, Box<dyn Error>> {
    let handler = scheduler::Scheduler::new(STATE_FILE_PATH);
    let server = server::Server::new(handler);
    let mut io = IoHandler::new();

    let address: SocketAddr = address.parse()?;
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io).start_http(&address)?;
    let close_handle = server.close_handle();

    std::thread::spawn(|| {
        server.wait();
    });

    Ok(close_handle)
}

pub fn list_resources() -> Vec<String> {
    // This is a dummy implementation that has already a formal implementation on another related
    // project
    let gpu_ids = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    gpu_ids
        .iter()
        .map(|id| format!("GPU{}", id))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = 2;
        let b = 2;
        assert_eq!(a + b, 4);
    }
}
