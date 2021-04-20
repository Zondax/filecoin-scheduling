mod client;
use client::*;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

extern crate clap;
use clap::{App, Arg, SubCommand};

const ADDRESS: &str = "127.0.0.1:5000";
const RATE: &str = "500";

fn monitoring(address: &str, rate: u64) {
    let client = Client::new(address).expect("Can not create rpc client");
    let rt = Runtime::new().unwrap();
    loop {
        match rt.block_on(async {
            let res = client.monitoring().await.unwrap();
            sleep(Duration::from_millis(rate as _)).await;
            res
        }) {
            Err(e) => {
                println!("Error {}", e);
                break;
            }
            Ok(res) => {
                println!();
                println!("--> Resources:");
                println!("        {:?}", res.resources);
                println!("--> Tasks:");
                println!("        {:?}", res.task_states);
                println!();
            }
        }
    }
}

fn abort(address: &str, id: u64) -> Result<(), String> {
    let client = Client::new(address).map_err(|e| e.to_string())?;
    let rt = Runtime::new().unwrap();
    rt.block_on(async { client.abort(id).await.map_err(|e| e.to_string()) })
        .map(|_| ())
}

fn main() -> Result<(), String> {
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

    let address = matches.value_of("address").unwrap_or(ADDRESS);

    if let Some(matches) = matches.subcommand_matches("monitor") {
        monitoring(
            address,
            (matches.value_of("rate").unwrap_or(RATE))
                .parse::<u64>()
                .map_err(|e| e.to_string())?,
        );
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("abort") {
        let id: &str = matches
            .value_of("client")
            .ok_or_else(|| "No client_id arg".to_string())?;
        let id = id.parse::<u64>().map_err(|e| e.to_string())?;
        abort(address, id)
    } else {
        Err("Niether monitor nor abort sub-command".to_string())
    }
}
