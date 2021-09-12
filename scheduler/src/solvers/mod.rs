mod greedy;
use crate::config::Settings;
use crate::solver::Solver;
pub use greedy::GreedySolver;

pub(crate) fn create_solver(_config: Option<&Settings>) -> Box<dyn Solver> {
    Box::new(GreedySolver)
}

#[cfg(dummy_devices)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::TaskState;
    use crate::{
        list_devices, solver::Resources, DeviceId, Pid, ResourceAlloc, ResourceMemory, ResourceReq,
        ResourceState, ResourceType, TaskRequirements,
    };
    use chrono::Utc;
    use std::collections::HashMap;
    use std::convert::TryFrom;
    use std::sync::atomic::{AtomicBool, AtomicU64};

    #[test]
    fn check_gpu_allocation() {
        let mut tasks = HashMap::new();
        let devices = list_devices();
        let state_t1 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task: None,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let mut devices_t1 = Resources(state_t1);

        let task1 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
                quantity: 1,
                preemptible: false,
            }],
            deadline: None,
            estimations: None,
            task_type: None,
        };

        let mut solver = create_solver(None);
        //can allocate on any device so go
        let alloc = solver.allocate_task(&mut devices_t1, &task1, &None, &tasks);

        assert!(alloc.is_some());

        let allocation = ResourceAlloc {
            requirement: task1.req[0].clone(),
            devices: vec![DeviceId::try_from("00:00").unwrap()],
        };
        let time: u64 = Utc::now().timestamp() as u64;
        let tasks_state = TaskState {
            requirements: task1.clone(),
            allocation: allocation.clone(),
            last_seen: AtomicU64::new(time),
            aborted: AtomicBool::new(false),
            creation_time: time,
            context: "".to_string(),
        };
        let state_t2 = devices
            .gpu_devices()
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let current_task = if i == 0 { Some(i as Pid) } else { None };
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let mut devices_t2 = Resources(state_t2);

        tasks.insert(1, tasks_state);

        //resource 0 is busy so should allocate on idle GPU instead
        let alloc = solver
            .allocate_task(&mut devices_t2, &task1, &None, &tasks)
            .unwrap();
        assert!(alloc.devices[0] != devices.gpu_devices()[0].device_id());

        let state_t3 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task: Some(0),
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let mut devices_t3 = Resources(state_t3);
        //everything busy so should allocate on any GPU instead
        let alloc = solver.allocate_task(&mut devices_t3, &task1, &None, &HashMap::new());
        assert!(alloc.is_some());

        let task2 = TaskRequirements {
            req: vec![
                ResourceReq {
                    resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
                    quantity: 2,
                    preemptible: false,
                },
                ResourceReq {
                    resource: ResourceType::Gpu(ResourceMemory::Mem(4)),
                    quantity: 1,
                    preemptible: false,
                },
            ],
            deadline: None,
            estimations: None,
            task_type: None,
        };

        let state_t4 = devices
            .gpu_devices()
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let current_task = if i == 0 { Some(i as Pid) } else { None };
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let mut devices_t4 = Resources(state_t4);
        let alloc = solver
            .allocate_task(&mut devices_t4, &task2, &None, &tasks)
            .unwrap();
        //allocate the requirement needing one idle GPU only instead of two of which one is busy
        assert!(alloc.devices[0] != devices.gpu_devices()[0].device_id());

        let task3 = TaskRequirements {
            req: vec![ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::Mem(4)),
                quantity: 1,
                preemptible: false,
            }],
            deadline: None,
            estimations: None,
            task_type: None,
        };

        let state_t5 = devices
            .gpu_devices()
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let current_task = if i == 0 { Some(i as Pid) } else { None };
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        current_task,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let mut devices_t5 = Resources(state_t5);
        let alloc = solver
            .allocate_task(
                &mut devices_t5,
                &task3,
                &Some(vec![devices.gpu_devices()[0].device_id()]),
                &tasks,
            )
            .unwrap();
        //allocate to 0 anyway since the task really needs to, even if it is busy..
        assert!(alloc.devices[0] == devices.gpu_devices()[0].device_id());
    }
}
