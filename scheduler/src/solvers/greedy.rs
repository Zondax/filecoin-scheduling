use std::collections::{HashMap, VecDeque};

use crate::config::Settings;
use crate::scheduler::task_is_stalled;
use crate::solver::{ResourceState, Resources, Solver, TaskState};
use crate::Error;
use common::{ResourceAlloc, ResourceMemory, ResourceType, TaskRequirements};

use priority_queue::PriorityQueue;
use rust_gpu_tools::opencl::DeviceUuid;
use std::cmp::Reverse;
use std::convert::TryFrom;

pub struct GreedySolver;
use std::sync::atomic::Ordering;

pub fn find_idle_gpus(resources: &Resources) -> Vec<DeviceUuid> {
    resources
        .0
        .iter()
        .filter(|(_, res)| !res.is_busy())
        .map(|(id, _)| *id)
        .collect::<Vec<DeviceUuid>>()
}

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
        restrictions: &Option<Vec<DeviceUuid>>,
    ) -> Option<(
        ResourceAlloc,
        std::collections::HashMap<DeviceUuid, ResourceState>,
    )> {
        // Use heuristic criteria for picking up a resource depending on task requirements
        // basing on the current resource load or even a greedy approach. For now we just take the
        // first that match and return

        let device_restrictions = restrictions
            .clone()
            .unwrap_or_else(|| resources.0.keys().cloned().collect::<Vec<DeviceUuid>>());

        let idle_gpus = find_idle_gpus(resources);
        // Make a new resource state, that the caller will use for updating the main resource state
        let mut resources = resources.0.clone();
        let mut options = vec![];
        for req in requirements.req.iter() {
            let quantity = req.quantity;
            // check if the pool of devices have room for the requested allocations
            let optional_resources = resources
                .iter_mut()
                .filter_map(|(index, device)| {
                    if let ResourceType::Gpu(ref mem) = req.resource {
                        match mem {
                            ResourceMemory::All => {
                                // Requesting all device memory is not an issue
                                // we assume the caller would handle the devices' memory
                                // management
                                Some(index.clone())
                            }
                            ResourceMemory::Mem(value) => {
                                if device.available_memory() >= *value {
                                    Some(index.clone())
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                })
                .filter(|b| device_restrictions.iter().any(|x| x == b))
                .collect::<Vec<DeviceUuid>>();
            let idle_gpus_available = optional_resources
                .iter()
                .cloned()
                .filter(|b| idle_gpus.iter().any(|x| x == b))
                .filter(|b| device_restrictions.iter().any(|x| x == b))
                .collect::<Vec<DeviceUuid>>();

            if idle_gpus_available.len() >= quantity {
                options = vec![(idle_gpus_available, req.clone())];
                break;
            } else if optional_resources.len() >= quantity {
                options.push((optional_resources, req.clone()));
            }
        }
        if !options.is_empty() {
            let selected_req = options[0].1.clone();
            let mut selected_resources = options[0].0.clone();
            selected_resources.truncate(selected_req.quantity as usize);
            selected_resources.iter().for_each(|index| {
                let _ = resources
                    .get_mut(index)
                    .map(|dev| dev.update_memory_usage(&selected_req.resource));
            });
            if let Ok(res) = selected_resources
                .into_iter()
                .map(|s| DeviceUuid::try_from(s))
                .collect::<Result<Vec<DeviceUuid>, _>>()
            {
                return Some((
                    ResourceAlloc {
                        requirement: selected_req,
                        resource_id: res,
                    },
                    resources,
                ));
            }
        }
        None
    }

    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
        scheduler_settings: &Settings,
    ) -> Result<VecDeque<u32>, Error> {
        //Criteria A: Use task end time as a priority indicator. The sooner the deadline the higher
        //the priority
        //
        //Criteria B: As a greedy algorithm, prioritize those that are consuming more resources in
        //term of memory and quantity
        //So that, we have a triplet (timestamp, memory_consumption, num_resources) as criteria for prioritazing

        // Use this handy crate that sort out our tasks
        let mut priority_queue = PriorityQueue::new();

        // iterate our tasks for making the triplet pushing it into the queue
        for (job_id, state) in input.iter() {
            // Intead of Reverse we can do something like deadline.end - chronos::now()?
            let is_stalled = Reverse(task_is_stalled(
                state.last_seen.load(Ordering::Relaxed),
                state.requirements.task_type,
                scheduler_settings,
            ));

            let deadline = state
                .requirements
                .deadline
                .map_or(i64::MAX, |d| d.end_timestamp_secs());
            let finish_time = Reverse(deadline);
            let mem_usage = match &state.allocation.requirement.resource {
                ResourceType::Gpu(mem) => match mem {
                    // This device is not preemptable so we put this at the end of our priority
                    // queue. Another option is removing it from the queue.
                    ResourceMemory::All => u64::MAX,
                    ResourceMemory::Mem(value) => {
                        value * state.allocation.requirement.quantity as u64
                    }
                },
                ResourceType::Cpu => unimplemented!("We handle just Gpu resources"),
            };
            let conditions = (
                is_stalled,
                finish_time,
                mem_usage,
                state.allocation.requirement.quantity,
            );
            priority_queue.push(job_id, conditions);
        }

        Ok(priority_queue
            .into_sorted_iter()
            .map(|(i, _)| *i)
            .collect::<VecDeque<u32>>())
    }
}
