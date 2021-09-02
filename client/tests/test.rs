use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use client::{
    spawn_scheduler_with_handler, Client, Error, ResourceAlloc, Settings, TaskFunc, TaskResult,
};
use common::{dummy_task_requirements, DeviceId, TaskType};

const NUM_ITERATIONS: usize = 20;

struct Test {
    index: usize,
    id: usize,
    devices_state: Arc<DevicesState>,
}

struct DevicesState(HashMap<DeviceId, AtomicBool>);
unsafe impl Sync for DevicesState {}

impl DevicesState {
    fn set_state(&self, id: &DeviceId, state: bool) {
        if self.0.get(id).unwrap().swap(state, Ordering::SeqCst) == state {
            panic!("Error: Multiple tasks using the same resource at the same time");
        }
    }
}

impl Test {
    fn new(id: usize, devices: Arc<DevicesState>) -> Self {
        Self {
            index: 0usize,
            id,
            devices_state: devices,
        }
    }
}

impl TaskFunc for Test {
    type Output = String;
    type Error = Error;

    fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error> {
        Ok(format!("Task {} done!!!", self.id))
    }

    fn task(&mut self, alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error> {
        let allocations = alloc.unwrap();
        // the task is allowed to continue so we set the resource that it uses to busy
        // mocking what the scheduler does internally
        for id in allocations.devices.iter() {
            self.devices_state.set_state(id, true)
        }

        let result = if self.index < NUM_ITERATIONS {
            self.index += 1;
            tracing::info!("Task {} >>> {} ", self.id, self.index);
            std::thread::sleep(Duration::from_millis(100));
            tracing::info!("Task {} <<<  ", self.id);
            TaskResult::Continue
        } else {
            tracing::info!("Task {} !!!  ", self.id);
            TaskResult::Done
        };

        // mark the resource as free
        for id in allocations.devices.iter() {
            self.devices_state.set_state(id, false)
        }

        Ok(result)
    }
}

#[test]
fn test_schedule() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let devices = common::list_devices();
    let mut hash_map = HashMap::new();

    devices.gpu_devices().iter().for_each(|dev| {
        hash_map.insert(dev.device_id(), AtomicBool::new(false));
        tracing::info!("Device {}", dev.name());
    });
    let devices_state = Arc::new(DevicesState(hash_map));

    let mut settings = Settings::new("/tmp/test.config.toml").unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    settings.service.address = format!("{}", socket.local_addr().unwrap());
    let handler =
        spawn_scheduler_with_handler(settings.clone(), "/tmp/schedule/", devices).unwrap();
    std::thread::sleep(Duration::from_millis(500));

    let mut joiner = vec![];

    for i in 0..4 {
        let state = devices_state.clone();
        let settings = settings.clone();
        joiner.push(std::thread::spawn(move || {
            let mut client = Client::register_with_settings::<Error>(settings).unwrap();
            client.set_context(format!("{}:{}", file!(), line!()));
            let mut test_func = Test::new(i as _, state);

            let mut task_req = dummy_task_requirements();
            task_req.task_type = Some(TaskType::MerkleTree);
            if i == 1 {
                task_req.task_type = Some(TaskType::WinningPost);
            }
            if i == 2 {
                task_req.task_type = Some(TaskType::WindowPost);
            }

            tracing::info!(
                "Task {} - pid {} <<<<<<<< {:?}",
                i,
                client.token.pid,
                task_req.req
            );
            client.schedule_one_of(&mut test_func, task_req, Duration::from_secs(60))
        }));
        std::thread::sleep(Duration::from_millis(10));
    }

    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    handler.close();
}
