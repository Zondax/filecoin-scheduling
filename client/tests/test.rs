use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rust_gpu_tools::opencl::GPUSelector;

use client::{
    register, schedule_one_of, spawn_scheduler_with_handler, Error, ResourceAlloc, TaskFunc,
    TaskResult,
};
use common::{dummy_task_requirements, TaskType};

const NUM_ITERATIONS: usize = 20;

struct Test {
    index: usize,
    id: usize,
    devices_state: Arc<DevicesState>,
}

struct DevicesState(HashMap<GPUSelector, AtomicBool>);
unsafe impl Sync for DevicesState {}

impl DevicesState {
    fn set_state(&self, id: &GPUSelector, state: bool) {
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
            std::thread::sleep(Duration::from_millis(500));
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

    let handler = spawn_scheduler_with_handler("127.0.0.1:5000", devices).ok();

    let mut joiner = vec![];

    for i in 0..4 {
        let state = devices_state.clone();
        joiner.push(std::thread::spawn(move || {
            let client = register::<Error>(None, Some(format!("{}:{}", file!(), line!()))).unwrap();
            let mut test_func = Test::new(i as _, state);

            let mut task_req = dummy_task_requirements();
            if i == 0 {
                task_req.task_type = Some(TaskType::MerkleProof);
                task_req.deadline = None;
            }
            if i == 1 {
                task_req.task_type = Some(TaskType::WindowPost);
                task_req.deadline = None;
            }
            if i == 2 {
                task_req.task_type = Some(TaskType::WinningPost);
                task_req.deadline = None;
            }

            tracing::info!("Task {} <<<<<<<< {:?}", i, task_req.req);
            schedule_one_of(client, &mut test_func, task_req, Duration::from_secs(60))
        }));
        std::thread::sleep(Duration::from_secs(2));
    }

    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        assert!(res.is_ok());
    }

    if let Some(h) = handler {
        h.close();
    }
}
