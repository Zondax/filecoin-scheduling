use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use client::*;
use tokio::runtime::Runtime;
use tokio::time::Duration;

use scheduler::MonitorInfo;

extern crate clap;
use clap::{App, Arg, SubCommand};

mod client;
#[allow(clippy::all)]
mod events;
mod gpu;
mod task;
mod tui;
mod util;

const ADDRESS: &str = "127.0.0.1:5000";
const RATE: &str = "500";

pub enum MonitorEvent {
    NewData(MonitorInfo),
    Abort,
}

fn monitoring(
    address: &str,
    rate: u64,
    sender: Sender<MonitorEvent>,
    recv: Receiver<MonitorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(address)?;
    let rt = Runtime::new().unwrap();
    loop {
        match recv.try_recv() {
            Ok(MonitorEvent::Abort) => return Err("Client close connection".into()),
            Err(TryRecvError::Disconnected) => return Err("Channel disconnected".into()),
            Ok(_) | Err(_) => {}
        }
        match rt.block_on(async { client.monitoring().await }) {
            Ok(res) => sender.send(MonitorEvent::NewData(res?))?,
            Err(_) => {}
        }

        std::thread::sleep(Duration::from_millis(rate as _));
    }
}

fn abort(address: &str, id: u64) -> Result<(), String> {
    let client = Client::new(address).map_err(|e| e.to_string())?;
    let rt = Runtime::new().unwrap();
    rt.block_on(async { client.abort(id).await.map_err(|e| e.to_string()) })
        .map(|_| ())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Monitor-app")
        .version("1.0")
        .about("Does awesome things")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("Service-address")
                .help("The service address - default: 127.0.0.1:5000")
                .required(false)
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("monitor")
                .about("Starts the monitoring service")
                .arg(
                    Arg::with_name("rate")
                        .short("r")
                        .long("rate")
                        .value_name("Rate")
                        .help("The update rate")
                        .required(false)
                        .takes_value(true)
                        .help("The interval in milliseconds - default value: 500ms"),
                ),
        )
        .subcommand(
            SubCommand::with_name("abort")
                .about("Make a request for aborting a client execution")
                .arg(
                    Arg::with_name("client")
                        .short("c")
                        .long("client")
                        .value_name("ClientId")
                        .help("The client id")
                        .required(true)
                        .takes_value(true)
                        .help("The client id whose resources are going to be released"),
                ),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap_or(ADDRESS).to_string();

    if let Some(matches) = matches.subcommand_matches("monitor") {
        let rate = (matches.value_of("rate").unwrap_or(RATE))
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        let (sender, recv) = channel();
        let (sender_abort, recv_abort) = channel();
        let monitor_handler = std::thread::spawn(move || {
            monitoring(&address, rate, sender, recv_abort).map_err(|e| e.to_string())
        });
        // launch the app and pass the recv half to it.
        tui::run_app(recv, sender_abort)?;
        match monitor_handler.join() {
            Ok(_) => Ok(()),
            Err(_) => Err("monitoring thread panics".into()),
        }
    } else if let Some(matches) = matches.subcommand_matches("abort") {
        let id: &str = matches
            .value_of("client")
            .ok_or_else(|| "The abort command requires a clientID".to_string())?;
        let id = id.parse::<u64>().map_err(|e| e.to_string())?;
        abort(&address, id).map_err(|e| e.into())
    } else {
        Err("Neither monitor nor abort sub-command".to_string().into())
    }
}
