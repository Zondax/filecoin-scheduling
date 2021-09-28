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
mod service;
mod solvers;
mod task;

pub use crate::config::Settings;
pub use crate::scheduler::Scheduler;
pub use client::ClientToken;
pub use db::Database;
pub use device::*;
pub use error::Error;
pub use handler::Handler;
pub use monitor::*;
pub use requests::{PreemptionResponse, RequestMethod};
pub use requirements::*;
pub use resource::*;
pub use service::{create_service, CloseService, HttpService, Service};
pub use task::{Pid, TaskState, TaskType};

use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

/// Starts a json-rpc server listening to *addr*
#[tracing::instrument(level = "debug", skip(devices, settings, database_path))]
pub fn run_scheduler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<()> {
    let db = Database::open(database_path, false)?;
    let handler = scheduler::Scheduler::new(settings.clone(), devices, db)?;
    let service = create_service::<Scheduler>(settings);
    service.start_service(handler)
}

#[tracing::instrument(level = "debug", skip(devices, settings, database_path))]
// To be use for testing purposes
pub fn spawn_scheduler_with_handler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<Box<dyn CloseService>> {
    let db = Database::open(database_path, true)?;
    let handler = scheduler::Scheduler::new(settings.clone(), devices, db)?;
    let service = create_service::<Scheduler>(settings);
    service.spawn_service(handler)
}
