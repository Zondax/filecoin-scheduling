#[cfg(feature = "greedy_solver")]
mod greedy;
#[cfg(feature = "mip_solver")]
mod linearsolver;
#[cfg(feature = "greedy_solver")]
pub use greedy::GreedySolver;
#[cfg(feature = "mip_solver")]
pub use linearsolver::{
    JobAllocation, JobConstraint, JobDescription, JobPlan, JobRequirements, LinearSolverModel,
};

use crate::config::Settings;
use crate::solver::Solver;
#[cfg(feature = "mip_solver")]
use common::TaskRequirements;

/// Wrapper struct for converting from TaskRequirements to
/// JobRequirements
#[cfg(feature = "mip_solver")]
pub struct RequirementsMap {
    pub reqs: TaskRequirements,
    // the available resources to use
    pub resources: Vec<u32>,
    pub job_id: usize,
    pub preemptive: Option<usize>,
    pub has_started: Option<(usize, usize)>,
}

#[cfg(feature = "mip_solver")]
impl From<RequirementsMap> for JobRequirements {
    fn from(map: RequirementsMap) -> Self {
        let options = map
            .resources
            .iter()
            .map(|id| JobConstraint {
                machine: *id as usize,
                duration: map.reqs.exec_time.as_secs() as usize,
            })
            .collect::<_>();
        let description = JobDescription {
            options,
            // We have a deadline wich contains the start/end times but need to
            // check corner cases or how they are going to be interpreted by the solver
            starttime: None,
            deadline: None,
            preemptive: None,
            has_started: None,
            is_support: false,
            job_id: map.job_id,
        };

        JobRequirements {
            jobs: vec![description],
            sequences: vec![],
            supports: vec![],
        }
    }
}

// Remove later this option, Config will have a default value, use it
#[cfg(feature = "mip_solver")]
pub(crate) fn create_solver(_config: Option<&Settings>) -> Box<dyn Solver> {
    Box::new(LinearSolverModel::new())
}

// TODO: Not sure about the optional settings here which is more like a scheduler-wise info
#[cfg(feature = "greedy_solver")]
pub(crate) fn create_solver(_config: Option<&Settings>) -> Box<dyn Solver> {
    Box::new(GreedySolver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::{ResourceState, Resources};
    use common::{ResourceMemory, ResourceReq, ResourceType, TaskRequirements};
    use std::collections::HashMap;

    #[test]
    fn check_gpu_allocation() {
        let devices = common::list_devices();
        println!("DEVICES: {:?}", devices);
        let state_t1 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: false,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t1 = Resources(state_t1);

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
        let alloc = solver.allocate_task(&devices_t1, &task1, &None);
        assert!(alloc.is_some());

        let state_t2 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: dev.device_id() == 0,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t2 = Resources(state_t2);

        //resource 0 is busy so should allocate on idle GPU instead
        let (alloc, _) = solver.allocate_task(&devices_t2, &task1, &None).unwrap();
        assert!(alloc.resource_id[0] != 0);

        let state_t3 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: true,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t3 = Resources(state_t3);
        //everything busy so should allocate on any GPU instead
        let alloc = solver.allocate_task(&devices_t3, &task1, &None);
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
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: dev.device_id() == 0,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t4 = Resources(state_t4);
        let (alloc, _) = solver.allocate_task(&devices_t4, &task2, &None).unwrap();
        //allocate the requirement needing one idle GPU only instead of two of which one is busy
        assert!(alloc.resource_id[0] != 0);

        let task3 = TaskRequirements {
            req: vec![
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

        let state_t5 = devices
            .gpu_devices()
            .iter()
            .map(|dev| {
                (
                    dev.device_id(),
                    ResourceState {
                        dev: dev.clone(),
                        mem_usage: 0,
                        is_busy: dev.device_id() == 0,
                    },
                )
            })
            .collect::<HashMap<_, ResourceState>>();
        let devices_t5 = Resources(state_t5);
        let (alloc, _) = solver.allocate_task(&devices_t5, &task3, &Some(vec![0])).unwrap();
        //allocate to 0 anyway since the task really needs to, even if it is busy..
        assert!(alloc.resource_id[0] == 0);
    }
}
