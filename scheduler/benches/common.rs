use scheduler::{list_devices, Database};
use scheduler::{Scheduler, Settings};

pub fn create_scheduler(name: &str) -> Result<Scheduler, scheduler::Error> {
    let settings = Settings::default();
    let devices = list_devices();
    let db = Database::open(name, true).unwrap();
    Scheduler::new(settings, devices, None, db)
}
