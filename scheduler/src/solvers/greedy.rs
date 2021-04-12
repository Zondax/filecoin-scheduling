use std::collections::{HashMap, VecDeque};

use crate::solver::{ResourceState, Resources, Solver, TaskState};
use crate::Error;
use common::{ResourceAlloc, ResourceMemory, ResourceType, TaskRequirements};

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub struct GreedySolver;

pub fn find_idle_gpus(resources: &Resources) -> Vec<u64> {
    resources
        .0
        .iter()
        .filter(|(_, res)| !res.is_busy())
        .map(|(id, _)| *id)
        .collect::<Vec<u64>>()
}

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
        restrictions: &Option<Vec<u64>>,
    ) -> Option<(ResourceAlloc, std::collections::HashMap<u64, ResourceState>)> {
        // Use heuristic criteria for picking up a resource depending on task requirements
        // basing on the current resource load or even a greedy approach. For now we just take the
        // first that match and return

        let device_restrictions = restrictions
            .clone()
            .unwrap_or(resources.0.keys().map(|x| *x).collect::<Vec<u64>>());

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
                                // All the memory means taking ownership of the device being also
                                // not preemptable. TODO: Should device be marked as no_shareable?
                                if device.mem_usage == 0 {
                                    //Some(*index, device.dev.device_id()))
                                    //Using a index instead of device_id, which varies on every
                                    //call
                                    Some(*index)
                                } else {
                                    None
                                }
                            }
                            ResourceMemory::Mem(value) => {
                                if device.available_memory() >= *value {
                                    Some(*index)
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
                .collect::<Vec<u64>>();
            let idle_gpus_available = optional_resources
                .iter()
                .cloned()
                .filter(|b| idle_gpus.iter().any(|x| x == b))
                .filter(|b| device_restrictions.iter().any(|x| x == b))
                .collect::<Vec<u64>>();

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
            return Some((
                ResourceAlloc {
                    requirement: selected_req,
                    resource_id: selected_resources,
                },
                resources,
            ));
        }
        None
    }

    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
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
            let condition = state
                .requirements
                .deadline
                .map_or(i64::MAX, |d| d.end_timestamp_secs());
            let finish_time = Reverse(condition);
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
            let triplet = (
                finish_time,
                mem_usage,
                state.allocation.requirement.quantity,
            );
            priority_queue.push(job_id, triplet);
        }

        Ok(priority_queue
            .into_sorted_iter()
            .map(|(i, _)| *i)
            .collect::<VecDeque<u32>>())
    }
}
